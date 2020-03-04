pub fn deserialize_label_message(mut vec: Vec<u8>) -> (String, Vec<u8>) {
    // get size data of label (vec) and label + message (label_and_message)
    let mut label_and_message = vec.split_off(8);

    // get size from vec
    let mut array = [0; 8];
    let bytes = &vec[..array.len()]; // panics if not enough data
    array.copy_from_slice(bytes);

    // split label and message
    let message = label_and_message.split_off(usize::from_be_bytes(array));

    (String::from_utf8(label_and_message).unwrap(), message)
}

pub fn serialize_label_message(label: String, msg: Vec<u8>) -> Vec<u8> {
    // put size of label
    let mut vec = usize::to_be_bytes(label.len()).to_vec();
    assert_eq!(8, vec.len());
    // put label
    vec.extend(label.as_bytes().to_vec());
    // put msg
    vec.extend(msg);

    vec
}

pub fn put_type(message_type: i8, bytes: &mut Vec<u8>) {
    bytes.insert(0, message_type as u8)
}

pub fn get_type(bytes: &mut Vec<u8>) -> i8 {
    bytes.remove(0) as i8
}
