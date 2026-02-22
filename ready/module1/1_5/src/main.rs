use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap(); // в конце операцию надо развернуть
    
    let amount: f32 = input.trim().parse().unwrap();

    if amount < 0.0 {
        println!("Ошибка! Сумма покупки не может быть отрицательной");
        return;
    }

    let discount: f32 = if amount >= 10000.0 {
        0.15
    } else if amount >= 5000.0 {
        0.1
    } else if amount >= 1000.0 {
        0.05
    } else {
        0.0
    };

    println!("Сумма покупки: {amount:.00} руб.");
    println!("Скидка: {:.0}%", discount * 100.0);
    println!("Итоговая стоимость: {:.2} руб.", amount * (1.0 - discount));
}
