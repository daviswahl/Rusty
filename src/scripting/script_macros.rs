
#[macro_export]
macro_rules! run {
    ($command:expr) => (
        //use rusty::core::execute::interpret; Need to make a conditional import
        println!("{}",execute::interpret($command.trim().split(' ').collect()));
    )
}

#[macro_export]
//Work on making this work for no input
macro_rules! cd {
    ($directory:expr) => (
        // use rusty::utils::cd::change_directory; Need to make a conditional import
        cd::change_directory($directory.trim().split(' ').collect());
    )
}
