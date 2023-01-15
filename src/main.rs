use std::io;

fn main() {
    let mut operacao = String::new();
    let list1 = ["+", "-", "/", "x", "potencia"];
    let list2 = ["raiz", "seno", "cosseno"];

    println!("Calculadora");
    println!("Qual das seguintes operações usaremos (+,-,/, x, potencia, raiz,seno ou cosseno ):");
    io::stdin().read_line(&mut operacao).unwrap();
    let operacao = operacao.trim();

    if list1.contains(&operacao) {
        let mut n1 = String::new();
        let mut n2 = String::new();
        println!("Qual é o primeiro valor:");
        io::stdin().read_line(&mut n1).unwrap();
        let n1: f64 = n1.trim().parse().unwrap();

        println!("Qual é o segundo valor:");
        io::stdin().read_line(&mut n2).unwrap();
        let n2: f64 = n2.trim().parse().unwrap();

        match operacao {
            "+" => {
                println!("A soma de {} por {} é {}", n1, n2, n1 + n2);
            }
            "-" => {
                println!("A subtração de {} por {} é {}", n1, n2, n1 - n2);
            }
            "/" => {
                println!("A divisão de {} por {} é {}", n1, n2, n1 / n2);
            }
            "x" => {
                println!("A multiplicação de {} por {} é {}", n1, n2, n1 * n2);
            }
            "potencia" => {
                println!("A potência de {} por {} é {}", n1, n2, n1.powf(n2));
            }
            _ => {
                println!("Resultado: 0.0");
            }
        }
    } else if list2.contains(&operacao) {
        let mut n1 = String::new();
        println!("Qual é o valor:");
        io::stdin().read_line(&mut n1).unwrap();
        let n1: f64 = n1.trim().parse().unwrap();

        match operacao {
            "raiz" => {
                println!("A raiz quadrada de {} é {}", n1, n1.sqrt());
            }
            "seno" => {
                let n2 = (n1.to_radians()).sin();
                println!("O seno de {} é {}", n1, format!("{:.2}", n2));
            }
            "cosseno" => {
                let n2 = (n1.to_radians()).cos();
                println!("O cosseno de {} é {}", n1, format!("{:.2}", n2));
            }
            _ => {
                println!("Resultado: 0.0");
            }
        }
    } else {
        print!("Valor invalido");
    }
}
