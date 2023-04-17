//
// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// main
//

mod ppm_interface;

fn main() -> std::io::Result<()> {
    let mut ppm = ppm_interface::PPMInterface::new(String::from("test.ppm"));
    ppm.write(1000, 1000);
    Ok(())
}


