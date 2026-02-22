use core::i32;

fn task_1_1() {
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
}

fn task_1_2() {
    /*
    Часть Б. Точки и векторы
    
    1. Создайте массив из 5 точек в 2D пространстве (каждая точка — кортеж (x, y)).
    2. Найдите точку, наиболее удаленную от начала координат (0, 0).
    3. Вычислите центр масс всех точек: (среднее X, среднее Y).
    */
    let points = [
        (1, 2), (3, 4),
        (5, 6), (7, 8),
        (9, 10)
    ];
    let points_len = points.len() as i32;
    
    let mut far_point_dist = 0;
    // Я использую индекс для хранения найденной точки.
    // Но поскольку набор данных небольшой и у них простая структура,
    // то можно использовать создание кортежа с копированием значения
    let mut far_point_idx = 0;
    
    let (mut mid_x, mut mid_y): (i32, i32) = (0, 0);
    for (i, &(x, y)) in points.iter().enumerate() {
        let point_dist = ((x * x + y * y) as i32).isqrt();
        if far_point_dist < point_dist {
            far_point_dist = point_dist;
            far_point_idx = i;
        }
    
        mid_x += x;
        mid_y += y;
    }
    
    let mid_point = (mid_x / points_len, mid_y / points_len);
    
    println!("\nЧасть Б");
    println!("Наиболее удаленная точка от (0, 0): {:?}", points[far_point_idx]);
    println!("Расстояние до этой точки: {far_point_dist}");
    println!("Центр масс всех точек: {:?}", mid_point);
}

fn task_1_3() {
    /* 
    Часть В. Срезы для анализа
    
    1. Создайте срез первых 3 точек из массива.
    2. Напишите функцию, которая принимает срез точек и возвращает:
        * Площадь ограничивающего прямоугольника (мин/макс X и Y)
        * В виде кортежа: (min_x, min_y, max_x, max_y)
    */

    let points = [
        (-2, 3),
        (7, -1),
        (-4, -8),
        (-5, -2),
        (-9, 5),
        (1, -6),
        (3, 4)
    ];

    let sliced = &points[..3];
    let (mut min_x, mut min_y): (i32, i32) = sliced[0];
    let (mut max_x, mut max_y): (i32, i32) = sliced[0];

    for &(x, y) in sliced {
        if min_x > x {
            min_x = x
        }
        if max_x < x {
            max_x = x;
        }
        if min_y > y {
            min_y = y
        }
        if max_y < y {
            max_y = y;
        }
    }

    let s = ((min_x - max_x) * (min_y - max_y)).abs();

    println!("\nЧасть В");
    println!("Точка минимума: {:?}. Точка максимума: {:?}", (min_x, min_y), (max_x, max_y));
    println!("Площадь ограничивающего прямоугольника: {s}");
}

fn main() {
    task_1_1();

    task_1_2();
    
    task_1_3();
}