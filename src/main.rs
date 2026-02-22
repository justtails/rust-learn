use core::i32;

fn main() {
    /*
    Часть А. Точки и векторы

    1. Создайте кортеж point_a, представляющий точку в 3D-пространстве (x, y, z).
    2. Создайте кортеж point_b с другими координатами.
    3. Вычислите вектор между точками (разность координат) и сохраните в кортеж vector_ab.
    4. Выведите длину вектора по формуле: √(x² + y² + z²) 
    */
    let point_a = (4, 5, 6);
    let point_b = (9, 8, 7);

    let vector_ab: (i32, i32, i32) = (
        point_b.0 - point_a.0, 
        point_b.1 - point_a.1, 
        point_b.2 - point_a.2
    );
    
    let vector_len = (vector_ab.0.pow(2) + vector_ab.1.pow(2) + vector_ab.2.pow(2)).isqrt();

    println!("Часть А");
    println!("Разность векторов: {:?} , длина вектора А: {vector_len}", vector_ab);

    /*
    Часть Б. Точки и векторы

    1. Создайте массив из 5 точек в 2D пространстве (каждая точка — кортеж (x, y)).
    2. Найдите точку, наиболее удаленную от начала координат (0, 0).
    3. Вычислите центр масс всех точек: (среднее X, среднее Y).
    */
    let points2d = [
        (1, 2), (3, 4),
        (5, 6), (7, 8),
        (9, 10)
    ];
    let points2d_len = points2d.len() as i32;

    let mut far_point_dist = 0;
    // Я использую индекс для хранения найденной точки.
    // Но поскольку набор данных небольшой и у них простая структура,
    // то можно использовать создание кортежа с копированием значения
    let mut far_point_idx = 0;

    let (mut mid_x, mut mid_y): (i32, i32) = (0, 0);
    for (i, &(x, y)) in points2d.iter().enumerate() {
        let point_dist = ((x * x + y * y) as i32).isqrt();
        if far_point_dist < point_dist {
            far_point_dist = point_dist;
            far_point_idx = i;
        }
    
        mid_x += x;
        mid_y += y;
    }

    let mid_point = (mid_x / points2d_len, mid_y / points2d_len);

    println!("\nЧасть Б");
    println!("Наиболее удаленная точка от (0, 0): {:?}", points2d[far_point_idx]);
    println!("Расстояние до этой точки: {far_point_dist}");
    println!("Центр масс всех точек: {:?}", mid_point);

    // Часть В
}
