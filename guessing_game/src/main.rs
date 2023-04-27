//biblioteca de Input/Output de variáveis
use std::io;
//biblioteca para gerar números aleatórios
use rand::Rng;
//biblioteca de comparação e ordenação
use std::cmp::Ordering;


fn main() {
    println!("Guess the number!");


    //Criação da variável e atribuindo o uso do rng e especificando o alcance que pode chegar de 1 até 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //Printando o valor gerado aleatóriamente
    //println!("The secret number is: {secret_number}");

    //Loop onde o usuário só para quando conseguir acertar o número gerado aleatóriamente
    loop{
        println!("Please input your guess: ");

        //Definindo variável mutável
        let mut guess = String::new();

        //Armazenando o input do usuário na variável (como se fosse o scanf em C)
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        /*
            Essa linha de código em Rust é uma declaração de uma nova variável guess do tipo u32 (unsigned integer de 32 bits).

            O que essa linha faz é:

            Lê uma string da entrada do usuário com a função std::io::stdin().read_line(&mut guess) e armazena o resultado na variável guess.

            A string lida pode conter espaços ou quebras de linha no final, então o método trim() é usado para remover quaisquer espaços em branco ou caracteres de quebra de 
            linha do início e do final da string.

            O método parse() converte a string lida em um número inteiro sem sinal u32, que é o tipo da variável guess. 
            Se a string lida não pode ser convertida em um número inteiro sem sinal u32, um erro será gerado.

            O método expect() é chamado para lidar com o caso em que parse() retorna um erro. Ele exibe a mensagem "Please type a number!" no console e interrompe o programa.

            Então, em resumo, essa linha de código lê uma entrada do usuário, converte-a em um número inteiro sem sinal u32 e armazena-o na variável guess. 
            Se a entrada não puder ser convertida em um número inteiro sem sinal u32, a mensagem "Please type a number!" será exibida no console e o programa será interrompido.
        
        
        
        */
        let guess: u32 = match guess.trim().parse(){

            //verifica se o input é um number ou não, se for ele passa (Ok) se não for ele volta e refaz a pergunta.
            Ok(num) => num,
            Err(_) => continue,
        };

        //Printa o número que o usuário colocou
        println!("You guessed: {guess}");

        //Comparação da variável &secret_number com a &guess e atribuindo os retornos de acordo com a comparação.
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win modafucker!");
                //break usado para parar o loop quando o input do usuário for igual ao número gerado aleatoriamente.
                break;
            },
        }
    }
}
