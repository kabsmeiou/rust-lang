// path differs depending on the operating system.
#[cfg(target_os = "windows")]
pub const REPO_PATH: &str = "C:/Users/fanta/Documents/GitHub/kabsmeiou.github.io/content";

#[cfg(target_os = "macos")]
pub const REPO_PATH: &str = "/Users/cerefrid/Documents/funspace/christiancabral.github.io/content";

#[cfg(target_os = "linux")]
pub const REPO_PATH: &str = "/home/cerefrid/Documents/code/kabsmeiou.github.io/content";