use crate::models::{Auditavel, Banco, Conta, ErroOperacao, Operacao};



impl Banco{
    pub fn novo(nome:String)->Banco{
        Banco{
            nome,
            contas: Vec::new(),
            historico: Vec::new(),
        }
    }



    pub fn encontrar_conta(&self, numero:u32)->Option<usize>{
        self.contas.iter().position(|c| c.numero == numero)
    }

    pub fn criar_conta(&mut self, numero:u32, titular:String){
        self.contas.push(Conta::nova(numero, titular.clone()));
        println!("Conta criada com sucesso: {} - {}", numero, titular);
    }


    pub fn depositar(&mut self, numero:u32, valor:f64)-> Result<(), ErroOperacao>{
        match self.encontrar_conta(numero){
            Some(indice)=>{
                self.contas[indice].depositar(valor)?;
                let op = Operacao{
                    numero_conta: numero,
                    tipo: "Depósito".to_string(),
                    valor,
                };
                op.registrar();
                self.historico.push(op);
                Ok(())
            }
            None=>{
                Err(ErroOperacao::ContaNaoEncontrada)
            }
        }
    }


    pub fn sacar(&mut self, numero:u32, valor:f64)-> Result<(), ErroOperacao>{
        match self.encontrar_conta(numero){
            Some(indice)=>{
                self.contas[indice].sacar(valor)?;
                let op = Operacao{
                    numero_conta: numero,
                    tipo: "Saque".to_string(),
                    valor,
                };
                op.registrar();
                self.historico.push(op);
                Ok(())
            }
            None=>{
                Err(ErroOperacao::ContaNaoEncontrada)
            }
        }
    }


    pub fn saldo(&self, numero: u32)-> Option<f64>{
        self.encontrar_conta(numero).map(|indice| self.contas[indice].ver_saldo())
    }

    pub fn ver_historico(&self){
        if self.historico.is_empty(){
            println!("Nenhuma operação realizada.");
        } else {
            for op in &self.historico {
                println!("Operação: {} - Valor: {}", op.tipo, op.valor);
            }
        }
    }


    pub fn listar_contas(&self){
        if self.contas.is_empty(){
            println!("Nenhuma conta cadastrada.");
        } else {
            for conta in &self.contas{
                println!("Conta: {} - Titular: {} - Saldo: {} - Ativa: {}", conta.numero, conta.titular, conta.saldo, conta.ativa);
            }
        }
    }

}
