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
    ];  // Массив сгенерировал для теста

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

fn analyze_books(books: &[(&str, &str, u16, u32, bool)]) -> (u32, u32, String) {
    let mut total_pages = 0_u32;
    let mut count_in_stock = 0_u32;
    let mut max_pages = books[0].3;
    let mut book_name = books[0].0;
    for (name, _, _, pages, in_stock) in books {
        total_pages += pages;

        if *in_stock {
            count_in_stock += 1;
        }

        if *pages > max_pages {
            max_pages = *pages;
            book_name = name;
        }
    }
    (total_pages, count_in_stock, book_name.to_string())
}

fn task_2() {
    /*
    Часть A: Информация о книге

    1. Создайте кортеж book, представляющий книгу:
        * Название (&str)
        * Автор (&str)
        * Год издания (u16)
        * Количество страниц (u32)
        * В наличии (доступна) (bool)

    Часть Б: Каталог книг

    1. Создайте массив из 6 книг (минимум 3 реальные).
    2. Найдите самую старую книгу в массиве.
    3. Посчитайте среднее количество страниц.

    Часть В. Работа со срезами

    1. Создайте срез книг, изданных после 2000 года.
    2. Напишите функцию, которая принимает срез книг и возвращает:
        * Общее количество страниц
        * Количество доступных книг
        * Название самой толстой книги
    */

    let books: [(&str, &str, u16, u32, bool); 6] = [
        ("Мастер и Маргарита", "Михаил Булгаков", 1967, 480, true),
        ("Преступление и наказание", "Фёдор Достоевский", 1866, 672, true),
        ("Война и мир", "Лев Толстой", 1869, 1225, false),
        ("1984", "Джордж Оруэлл", 1949, 328, true),
        ("Маленький принц", "Антуан де Сент-Экзюпери", 1943, 96, true),
        ("Солярис", "Станислав Лем", 1961, 320, false)
    ];

    let mut oldest_book = books[0];
    let mut mid_pages = books[0].3;

    for &book @ (_, _, year, pages, _) in &books[1..] {
        if year < oldest_book.2 {
            oldest_book = book;
        }
        
        mid_pages += pages;
    }

    mid_pages = mid_pages / (books.len() as u32);
    let result = analyze_books(&books[1..4]);

    println!("\nЗадание 2");
    println!("Самая старая книга: {:?}", oldest_book);
    println!("Среднее число страниц: {mid_pages}");
    println!("Изучение среза
    Общее количество страниц в срезе: {}
    Количество доступных книг: {}
    Самая толстая книга: {}",
    result.0, result.1, result.2
    );
}

fn analyze_measurement(
    slice1: &[(&str, &str, i8, i8)], 
    slice2: &[(&str, &str, i8, i8)]
) -> (String, i8, u8) {
    let mut mid_dtemp1: (&str, i16) = (slice1[0].0, slice1[0].2 as i16);
    let mut mid_dtemp2: (&str, i16) = (slice2[0].0, slice2[0].2 as i16);
    let mut max_diff = (slice1[0].2 - slice2[0].2).abs();
    let mut count: u8 = 0;
    
    let mut result: (String, i8, u8) = ("".to_string(), 0, 0);

    for (a, b) in slice1[1..].iter().zip(slice2[1..].iter()) {
        let diff = (a.2 - b.2).abs();
        if diff > max_diff {
            max_diff = diff;
        }

        if ((b.2 + b.3) / 2) > ((a.2 + a.3) / 2) {
            count += 1;
        }
        
        mid_dtemp1.1 += a.2 as i16;
        mid_dtemp2.1 += b.2 as i16;
    }
    result.1 = max_diff.abs();
    result.2 = count;

    mid_dtemp1.1 /= slice1.len() as i16;
    mid_dtemp2.1 /= slice2.len() as i16;
    if mid_dtemp1 > mid_dtemp2 {
        result.0 = mid_dtemp1.0.to_string();
    } else {
        result.0 = mid_dtemp2.0.to_string();
    }

    return result;
}

fn task_3() {
    /*
    Задача 3. Анализ температурных данных
    Часть A: Измерения температуры
    
    Создайте кортеж measurement, представляющий измерение:
        * Город (&str)
        * Дата в формате "ДД.ММ" (&str)
        * Температура днем (i8)
        * Температура ночью (i8)
    
    Часть Б: Недельный прогноз
    
    1. Создайте массив из 7 измерений для одного города (неделя).
    2. Найдите самый теплый и самый холодный день (по дневной температуре).
    3. Вычислите среднюю дневную и среднюю ночную температуру за неделю.
    
    Часть В: Сравнение городов
    
    1. Создайте два среза: измерения для Москвы и для Сочи (по 7 дней в каждом).
    2. Напишите функцию, которая сравнивает два среза измерений и возвращает:
        * В каком городе теплее в среднем (по дневной температуре)
        * Максимальную разницу температур между городами в один день
        * Количество дней, когда в Сочи было теплее, чем в Москве
    */

    let measurement: [(&str, &str, i8, i8); 14] = [
        ("Москва", "01.01", -8, -12),
        ("Москва", "02.01", -5, -10),
        ("Москва", "03.01", -3, -7),
        ("Москва", "04.01", -10, -15),
        ("Москва", "05.01", -12, -18),
        ("Москва", "06.01", -7, -11),
        ("Москва", "07.01", -4, -9),
        ("Сочи", "01.01", 5, 1),
        ("Сочи", "02.01", 8, 3),
        ("Сочи", "03.01", 6, 2),
        ("Сочи", "04.01", 3, -1),
        ("Сочи", "05.01", 4, 0),
        ("Сочи", "06.01", 7, 4),
        ("Сочи", "07.01", 9, 5)
    ];

    // Решение часть Б
    let list1 = &measurement[..7];
    let list2 = &measurement[7..];
    let list1_len = list1.len() as i32;
    
    let mut maxtemp_day = list1[0];
    let mut mintemp_day = list1[0];
    let mut mid_dtemp = list1[0].2 as i32;
    let mut mid_ntemp = list1[0].3 as i32;
    
    for &day @ (_, _, dtemp, ntemp) in &list1[1..] {
        if dtemp < mintemp_day.2 {
            mintemp_day = day;
        }
        if dtemp > maxtemp_day.2 {
            maxtemp_day = day;
        }

        mid_dtemp += dtemp as i32;
        mid_ntemp += ntemp as i32;
    }

    mid_dtemp /= list1_len;
    mid_ntemp /= list1_len;

    // Решение часть В
    let result = analyze_measurement(list1, list2);

    println!("\nЗадание 3");
    println!("Самый теплый день - {}, дн. температура {} C", maxtemp_day.1, maxtemp_day.2);
    println!("Самый холодный день - {}, дн. температура {} C", mintemp_day.1, mintemp_day.2);
    println!("Средняя дневная температура: {mid_dtemp}");
    println!("Средняя ночная температура: {mid_ntemp}");
    println!("Теплее всего в городе {}", result.0);
    println!("Максимальная разница температур была {}", result.1);
    println!("В Сочи было теплее {} дней", result.2);
}

fn main() {
    task_1_1();

    task_1_2();
    
    task_1_3();

    task_2();

    task_3();
}

