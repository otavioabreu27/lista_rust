use std::{io, str::FromStr, fmt::Debug};

fn main() {
  ex_6();
}

// fn ex_1() {
//   // Faça um Programa que peça dois números e imprima o maior deles.
//   let num1: i32 = prompt("Digite o primeiro número: ");
//   let num2: i32 = prompt("Digite o segundo número: ");

//   if num1 >= num2 {
//     println!("{}", num1);
//   } else {
//     println!("{}", num2);
//   }
// }

// fn ex_2() {
//   // Faça um Programa que peça um valor e mostre na tela se o valor é positivo ou negativo. 
//   let num: f64 = prompt("Digite o valor:");

//   if num < 0.0 {
//     println!("É negativo");
//   } else if num > 0.0 {
//     println!("É positivo");
//   } else {
//     println!("É zero");
//   }
// }

// fn ex_3() {
//   // Faça um Programa que verifique se uma letra digitada é "F" ou "M". 
//   // Conforme a letra escrever: F - Feminino, M - Masculino, Sexo Inválido.
//   let sexo: char = prompt("Digite F para Feminino e M para Masculino:");

//   if sexo == 'M' {
//     println!("Masculino");
//   } else if sexo == 'F' {
//     println!("Feminino");
//   } else {
//     println!("Sexo inválido");
//   }
// }

// fn ex_4() {
//   // Faça um Programa que verifique se uma letra digitada é vogal ou consoante. 
//   let array_vogais: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
//   let letra: char = prompt("Escreva uma letra: ");
  
//   if array_vogais.contains(&letra) {
//     println! ("É vogal");
//   } else {
//     println! ("É consoante");
//   }
// }

// fn ex_5() {
//   // Faça um programa para a leitura de duas notas parciais de um aluno. 
//   // O programa deve calcular a média alcançada por aluno e apresentar:
//   //  - A mensagem "Aprovado", se a média alcançada for maior ou igual a sete;
//   //  - A mensagem "Reprovado", se a média for menor do que sete;
//   //  - A mensagem "Aprovado com Distinção", se a média for igual a dez. 

//   let nota1: f32 = prompt("Digite a nota do primeiro bimestre: ");
//   let nota2: f32 = prompt("Digite a nota do segundo bimestre: ");
//   let nota3: f32 = prompt("Digite a nota do terceiro bimestre: ");
//   let nota4: f32 = prompt("Digite a nota do quarto bimestre: ");

//   let media: f32 = (nota1 + nota2 + nota3 + nota4) / 4.0;

//   println!("{}", media);

//   if media == 10.0 {
//     println!("Aprovado com Distinção")
//   } else if media >= 7.0 {
//     println!("Aprovado!");
//   } else  {
//     println!("Reprovado");
//   }
// }

fn ex_6() {
  // Faça um Programa que leia três números e mostre-os em ordem decrescente. 
  let num1: i32 = prompt("Digite o primeiro número: ");
  let num2: i32 = prompt("Digite o segundo número: ");
  let num3: i32 = prompt("Digite o terceiro número: ");

  let mut vec: Vec<i32> = vec![num1, num2, num3];

  vec.sort_by(|a, b| b.cmp(a));
  
  let mut cont: i32 = 0;

  for value in &vec {
    println!("{}º número: {}", cont, value);
    cont += 1;
  }
}

fn prompt<T: std::str::FromStr>(prompt_message: &str) -> T where <T as FromStr>::Err: Debug {
  println!("{}", prompt_message);

  let mut input = String::new();
  io::stdin()
      .read_line(&mut input)
      .expect("Failed to read input");

  input.trim().parse().expect("Failed to parse input")
}