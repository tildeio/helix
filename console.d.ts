interface ConsoleClass {
  new(): Console;
}

interface Console {
  log(message: string): void;
}

// 1. Get init running
// 2. Export a global variable (number)
// 3. Make a module / class
// 4. Create a method that expects a simple type and does work (log(RubyString))
// 5. Have the Rust code (safely) call back into Ruby to get the type it needs
