use std::io;

fn main() {
  ex_7();
}

// fn ex_1() {
//   // Faça um Programa que mostre a mensagem "Alo mundo" na tela. 
//   println!("Alo mundo");
// }

// fn ex_2(stdin: io::Stdin) {
//   // Faça um Programa que peça um número e então mostre a mensagem O número informado foi [número]. 
//   let mut num = String::new();
//   println!("Digite um número: ");
//   stdin
//     .read_line(&mut num)
//     .expect("Failed to read the line");
//   println!("O numero informado foi: {}", num);
// }

// fn ex_3() {
//   //Faça um Programa que peça dois números e imprima a soma.
//   let prompt_num1 = prompt("Digite o primeiro número:");
//   let prompt_num2 = prompt("Digite o segundo número:");

//   let num1: u32 = to_integer(prompt_num1);
//   let num2: u32 = to_integer(prompt_num2);

//   println!("A soma dos valores é {}", (num1 + num2)); 
// }

// fn ex_4() {
//   // Faça um Programa que peça as 4 notas bimestrais e mostre a média.
//   let nota1: f32 = to_float(prompt("Digite a primeira nota: "));
//   let nota2: f32 = to_float(prompt("Digite a segunda nota: "));
//   let nota3: f32 = to_float(prompt("Digite a terceira nota: "));
//   let nota4: f32 = to_float(prompt("Digite a quarta nota: "));

//   let media: f32 = (nota1 + nota2 + nota3 + nota4) / 4.0;

//   println!("A média do aluno é: {}", media)
// }

// fn ex_5() {
//   // Faça um Programa que converta metros para centímetros. 

//   let metros: f32 = to_float(prompt("Digite o valor em metros: "));

//   println!("A medida em cm é: {}", (metros * 100.0));
// }

// fn ex_6() {
//   // Faça um Programa que peça o raio de um círculo, calcule e mostre sua área.
//   let pi = std::f32::consts::PI;

//   let raio: f32 = to_float(prompt("Digite o raio do círculo: "));

//   println!("A área do círculo é: {}", (pi * raio.powi(2)));
// }

fn ex_7() {
  // Faça um Programa que calcule a área de um quadrado, em seguida mostre o dobro desta área para o usuário.
  let lado_quadrado: f32 = to_float(prompt("Digite o valor do lado do quadrado: "));

  let area: f32 = lado_quadrado.powi(2);

  println!("O valor da área é {}, o dobro disso é {}.", area, area * 2.0)
}

fn prompt(prompt_message: &str) -> String {
  let mut prompt_result = String::new();

  println!("{}", prompt_message);

  io::stdin()
    .read_line(&mut prompt_result)
    .expect("Erro ao ler a inserção");

  return prompt_result;
 }

// fn to_integer(string_value: String) -> u32 {
//   let integer_value = string_value
//     .trim()
//     .parse()
//     .expect("Erro ao transformar em inteiro");

//   return integer_value;
// }

  fn to_float(string_value: String) -> f32 {
    let float_value = string_value
      .trim()
      .parse::<f32>()
      .expect("Erro ao transformar em inteiro");

    return float_value;
  }