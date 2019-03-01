mod controler

pub struct Controler{
    io : io::Io,
    handlers : Arc<Mutex< HashMap<String,fn(String)> >>,
    default_handler : Arc<Mutex<fn(String)>>,
}
