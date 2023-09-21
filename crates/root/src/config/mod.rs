



pub struct Config {
    pub(crate) port: usize
}





static mut CONFIG: Config = Config {
    port: 8956
};


pub fn set(config: Config) {
    unsafe {
        CONFIG = config
    }

    // std::env::var("home");


}

pub fn get() {



}

pub fn init(config: Config) {
    set(config);
    sync();
}

pub fn sync() {

}
