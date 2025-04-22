use does_it_fit::*;

#[test]
fn no_volumes_shapes() {
    assert!(area_fit((2, 5), GeometricalShapes::Circle, 0, (2, 1)));
    assert!(area_fit((2, 2), GeometricalShapes::Rectangle, 0, (6, 10)));
    assert!(volume_fit(
        (2, 5, 3),
        GeometricalVolumes::Cone,
        0,
        (1, 1, 1)
    ));
    assert!(volume_fit(
        (3, 5, 7),
        GeometricalVolumes::Parallelepiped,
        0,
        (2, 6, 3)
    ));
}

#[test]
fn equal_size() {
    assert!(area_fit((2, 5), GeometricalShapes::Square, 1, (2, 5)));
    assert!(volume_fit(
        (3, 1, 4),
        GeometricalVolumes::Cube,
        1,
        (1, 3, 4)
    ));
}

#[test]
fn all_shapes() {
    assert!(!area_fit((3, 5), GeometricalShapes::Circle, 2, (2, 0)));
    assert!(area_fit((8, 6), GeometricalShapes::Triangle, 5, (5, 2)));
    assert!(area_fit((7, 3), GeometricalShapes::Rectangle, 2, (2, 4)));
    assert!(area_fit((5, 5), GeometricalShapes::Square, 1, (2, 4)));
}

#[test]
fn all_volumes() {
    assert!(volume_fit(
        (5, 6, 3),
        GeometricalVolumes::Cube,
        2,
        (3, 3, 4)
    ));
    assert!(!volume_fit(
        (7, 4, 4),
        GeometricalVolumes::Cone,
        1,
        (8, 2, 0)
    ));
    assert!(volume_fit(
        (2, 5, 3),
        GeometricalVolumes::Sphere,
        1,
        (1, 1, 1)
    ));
    assert!(!volume_fit(
        (2, 5, 3),
        GeometricalVolumes::Parallelepiped,
        31,
        (1, 1, 1)
    ));
    assert!(volume_fit(
        (7, 5, 3),
        GeometricalVolumes::TriangularPyramid,
        3,
        (3, 2, 1)
    ));
}
