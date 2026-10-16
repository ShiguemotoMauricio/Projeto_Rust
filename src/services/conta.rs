use crate::models::{Conta, ErroOperacao};

impl Conta{
    pub fn nova(numero:u32, titular:String)->Conta{    
        Conta{
            numero,
            titular,
            saldo: 0.0,
            ativa: true,
        }
    }

    pub fn ver_saldo(&self)->f64{
    self.saldo
    }

    pub fn depositar(&mut self, valor: f64)-> Result<(), ErroOperacao>{
        if!self.ativa{
            return Err(ErroOperacao::ContaInativa);
        }

        if valor<=0.0{
            return Err(ErroOperacao::ValorInvalido);
        }

        self.saldo+=valor;
        Ok(())

    }

    pub fn sacar(&mut self, valor: f64)-> Result<(), ErroOperacao>{
        if!self.ativa{
            return Err(ErroOperacao::ContaInativa);
        }

        if valor<=0.0{
            return Err(ErroOperacao::ValorInvalido);
        }

        if self.saldo<valor{
            return Err(ErroOperacao::SaldoInsuficiente);
        }

        self.saldo-=valor;

        Ok(())
    }
}

