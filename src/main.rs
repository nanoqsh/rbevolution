mod chart;

fn main() {
    let data = vec![
        116, 129, 135, 86, 73, 85, 73, 68, 92, 130, 245, 139, 115, 111, 309, 206, 137, 128, 85, 94,
        71, 106, 84, 93, 85, 73, 83, 125, 107, 82, 44, 72, 106, 107, 66, 91, 92, 113, 107, 131,
        111, 64, 69, 88, 77, 83, 111, 57, 55, 60, 71, 106, 84, 93, 85, 73, 83, 125, 107, 82, 44,
        72, 106, 107, 66, 91, 92, 113, 107, 131, 111, 64, 69, 88, 77, 83, 111, 57, 55, 60, 07, 82,
        44, 72, 106, 107, 66, 91, 92, 113, 107, 131,
    ];

    crate::chart::render(data);
}