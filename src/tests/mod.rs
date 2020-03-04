#![cfg(test)]

use crate::node::Builder;
use crate::serializers::*;

use crossbeam::crossbeam_channel::unbounded;

#[test]
fn test_serializers() {
    let label: String = String::from("Ola");

    let msg: Vec<u8> = "mundo lindo".as_bytes().to_vec();
    let my_type: i8 = 2;

    // Serialize //
    let mut bytes = serialize_label_message(label.clone(), msg.clone());
    put_type(my_type, &mut bytes);

    // desserialize //
    let my_type_res = get_type(&mut bytes);
    let (label_res, msg_res) = deserialize_label_message(bytes);

    assert_eq!(msg, msg_res);

    assert_eq!(label, label_res);

    assert_eq!(my_type, my_type_res);
}

#[test]
fn test_controller() {
    let (s1, r1) = unbounded();
    let (s2, r2) = unbounded();
    let (s3, r3) = unbounded();

    let h1 = move |v: Vec<u8>| {
        s1.send(v.clone()).unwrap();
    };
    let h2 = move |v: Vec<u8>| {
        s2.send(v.clone()).unwrap();
    };
    let h3 = move |v: Vec<u8>| {
        s3.send(v.clone()).unwrap();
    };

    let node = Builder::new(11101)
        .set_simple_controller(h1)
        .set_label_controller(h2)
        .add_label_handler("label".to_string(), h3)
        .build(3);

    let vec_1 = "my first message".as_bytes().to_vec();
    let vec_2 = "another message".as_bytes().to_vec();
    let vec_3 = "random message".as_bytes().to_vec();
    let vec_4 = "last message".as_bytes().to_vec();

    node.send(vec_1.clone(), "localhost:11101".to_string());

    node.send(vec_2.clone(), "localhost:11101".to_string());

    node.send_with_label(
        vec_3.clone(),
        "label".to_string(),
        "localhost:11101".to_string(),
    );

    node.send_with_label(
        vec_4.clone(),
        "randomLabel".to_string(),
        "localhost:11101".to_string(),
    );

    assert_eq!(r1.recv(), Ok(vec_1));
    assert_eq!(r1.recv(), Ok(vec_2));
    assert_eq!(r3.recv(), Ok(vec_3));
    assert_eq!(r2.recv(), Ok(vec_4));
}

#[test]
fn test_mut_controller() {
    let mut id = 0;

    let (s, r) = unbounded();

    let h = move |v: Vec<u8>| {
        let mut message = id.to_string().as_bytes().to_vec();
        id += 1;
        let mut clone = v.clone();
        message.append(&mut "-".as_bytes().to_vec());
        message.append(&mut clone);

        s.send(message).unwrap();
    };

    let node = Builder::new_with_dns(3031, "./src/tests/dns.txt".to_string())
        .set_simple_controller_mut(h)
        .build(1);

    let message = "ola mundo".as_bytes().to_vec();

    for _i in 0..4 {
        node.send(message.clone(), "me".to_string());
    }

    for i in 0..4 {
        let v = r.recv().unwrap();
        assert_eq!(v, format!("{}-ola mundo", i).as_bytes().to_vec());
    }
}

#[test]
fn test_data_workflow() {
    let node_builder = Builder::new(11103);
    let counter;
    let upper;
    let duplicate;
    let default;

    let (s, r) = unbounded();

    {
        let node_config = node_builder.get_shallow_node();
        let mut count = 0;
        counter = move |mut v: Vec<u8>| {
            let mut message = count.to_string().as_bytes().to_vec();
            count += 1;
            message.append(&mut v);
            node_config.send_with_label(
                message,
                "upper".to_string(),
                "localhost:11103".to_string(),
            );
        };
    }
    {
        let node_config = node_builder.get_shallow_node();
        upper = move |v: Vec<u8>| {
            let message = String::from_utf8(v).unwrap().to_uppercase();
            node_config.send_with_label(
                message.as_bytes().to_vec(),
                "duplicate".to_string(),
                "localhost:11103".to_string(),
            );
        };
    }
    {
        let node_config = node_builder.get_shallow_node();
        duplicate = move |v: Vec<u8>| {
            let message = String::from_utf8(v).unwrap().repeat(2);
            node_config.send_with_label(
                message.as_bytes().to_vec(),
                "none".to_string(),
                "localhost:11103".to_string(),
            );
        };
    }

    default = move |v: Vec<u8>| {
        let mut vv = v.clone();
        vv.push('f' as u8);
        s.send(vv).unwrap();
    };

    let final_confi = node_builder
        .set_simple_controller_mut(counter)
        .set_label_controller(default)
        .add_label_handler("duplicate".to_string(), duplicate)
        .add_label_handler("upper".to_string(), upper)
        .build(2);

    let text = "Hello world";
    let mut results = Vec::new();

    for i in 0..4 {
        let message = format!("{}{}", text, i).as_bytes().to_vec();
        results.push(format!(
            "{}{}{}{}{}{}f",
            i,
            text.to_uppercase(),
            i,
            i,
            text.to_uppercase(),
            i
        ));
        final_confi.send(message, "localhost:11103".to_string());
    }

    for _i in 0..4 {
        let s = String::from_utf8(r.recv().unwrap()).unwrap();
        assert!(results.contains(&s));
    }
}

#[test]
fn test_mut_label() {
    let (s, r) = unbounded();

    let mut my_string = String::new();

    let h = move |v: Vec<u8>| {
        if v.len() == 0 {
            s.send(my_string.clone()).unwrap();
        } else {
            my_string.push_str(&String::from_utf8(v).unwrap());
        }
    };

    Builder::new(3331)
        .set_label_controller_mut(|_v: Vec<u8>| assert!(false))
        .add_label_handler_mut("label".to_string(), h)
        .build(1);

    let config = Builder::new(3332).build(0);

    let text = "hello";

    let num = 4;

    for _i in 0..num {
        config.send_with_label(
            text.as_bytes().to_vec(),
            "label".to_string(),
            "localhost:3331".to_string(),
        );
    }

    config.send_with_label(
        Vec::new(),
        "label".to_string(),
        "localhost:3331".to_string(),
    );

    assert_eq!("hello".repeat(num), r.recv().unwrap());
}
