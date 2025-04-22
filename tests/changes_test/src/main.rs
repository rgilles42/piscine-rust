use changes::*;

fn main() {
    let mut lights = ["living_room", "bedroom", "rest_room"].map(Light::new);

    println!("brightness = {}", lights[0].brightness);

    change_brightness(&mut lights, "living_room", 200);

    println!("new brightness = {}", lights[0].brightness);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alias() {
        let mut lights = (0..5)
            .map(|i| Light::new(&format!("light-{i}")))
            .collect::<Vec<_>>();

        let alias = "light-3";
        change_brightness(&mut lights, alias, 100);
        assert_eq!(lights[3].brightness, 100);
    }

    #[test]
    fn test_nonexistent_alias() {
        let mut lights = (0..5)
            .map(|i| Light::new(&format!("light-{i}")))
            .collect::<Vec<_>>();

        let copy = lights.clone();
        change_brightness(&mut lights, "light-6", 100);
        assert_eq!(copy, lights);
    }
}
