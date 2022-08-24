fn main() {
  struct Account {
    money: i8,
    name: String
  }

  trait Operation {
    fn withdraw(&mut self, value: i8);
    fn deposit(&mut self, value: i8);
    fn extract(&self);
  }

  impl Operation for Account {
    fn deposit(&mut self, value: i8) {
      self.money = self.money + value;
    }
    fn withdraw(&mut self, value: i8) {
      self.money = self.money - value;
    }
    fn extract(&self) {
      println!("{} have {}", self.name, self.money);
    }
  }

  let my_name: String = String::from("Lucas Marcel");
  let other_name: String = String::from("Alguém");

  let mut my_account: Account = Account{ money: 5, name: my_name};
  let mut other_account: Account = Account{ money: 8, name: other_name };

  my_account.deposit(20);
  my_account.extract();
  my_account.withdraw(6);
  my_account.extract();

  other_account.deposit(8);
  other_account.deposit(5);
  other_account.extract();
} 