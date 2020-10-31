import { Context } from "./core/context.ts";
import { Interpreter } from "./core/interpreter.ts";
import { Lexer } from "./core/lexer.ts";
import { Number } from "./core/number.ts";
import { Parser } from "./core/parser/parser.ts";
import { Err } from "./error/err.ts";
import { SymbolTable } from "./utils/symbol_table.ts";
import { Variable } from "./utils/variable.ts";

const global = new SymbolTable();
global.set("true", new Variable<Number>(new Number(1), "Int", false));
global.set("false", new Variable<Number>(new Number(1), "Float", false));
const context = new Context("<Global>");
context.symbolTable = global;

/**
 * @param name File name, reads code from the file if code not provided
 * @param code Optional code to evaluate
 * @description Runs a BlazeScript Code
 */
export function run(
  name: string,
  code?: string,
): { interpreted?: Number | null; error?: Err | null; errors?: Err[] | null } {
  try {
    const { tokens, errors } = new Lexer(
      name,
      code ?? new TextDecoder("utf-8").decode(Deno.readFileSync(Deno.args[0])),
    ).makeTokens();
    if (errors) {
      console.log(errors.map((error) => error.formatted()).join(", \n"));
      return { errors };
    }

    const { node, error } = new Parser(tokens!).parse();
    if (error) {
      console.error(error.formatted());
      return { error };
    }

    const interpreter = new Interpreter();
    const { value, error: rterr } = interpreter.visit(node!, context);
    if (rterr) {
      console.log(rterr.formatted());
      return { error: rterr };
    }
    console.log(value?.represent());
    return { interpreted: value! };
  } catch (e) {
    throw "Unexpected Error Given by Deno.\nPlease open a issue at https://github.com/RoMeAh/blazescript/issues/ with description of\n" +
      (e.stack ?? e);
  }
}

if (!Deno.args[0]) {
  while (true) {
    await Deno.stdout.write(new TextEncoder().encode("blazescript > "));
    const buf = new Uint8Array(128);
    const n = await Deno.stdin.read(buf);
    const message = new TextDecoder().decode(buf.subarray(0, n ?? undefined));
    if (!message.startsWith(".")) {
      run("<stdin>", message);
    } else {
      const args = message.replace(".", "").split(" ");
      if (args[0] == "exit\r\n") {
        Deno.exit();
      } else {
        await Deno.stdout.write(
          new TextEncoder().encode(`Unknown command ${args.join(" ")}`),
        );
      }
    }
  }
}

if (Deno.args[0] == "-e") {
  const args = Deno.args.map((a) => a);
  args.shift();
  run("Eval", args.join(" "));
  Deno.exit();
} else if (Deno.args[0] == "--eval") {
  const args = Deno.args.map((a) => a);
  args.shift();
  run("Eval", args.join(" "));
  Deno.exit();
} else if (Deno.args[0] == "--repl") {
  while (true) {
    await Deno.stdout.write(new TextEncoder().encode("blazescript > "));
    const buf = new Uint8Array(128);
    const n = await Deno.stdin.read(buf);
    const message = new TextDecoder().decode(buf.subarray(0, n ?? undefined));
    if (!message.startsWith(".")) {
      run("<stdin>", message);
    } else {
      const args = message.replace(".", "").split(" ");
      if (args[0] == "exit\r\n") {
        Deno.exit();
      } else {
        console.log("Unknown command!");
      }
    }
  }
}

const fileOrFolder = Deno.args[0];
if (fileOrFolder.endsWith(".bzs")) {
  run(fileOrFolder);
} else {
  readDir(fileOrFolder);
  function readDir(dirName: string) {
    for (const dirEntry of Deno.readDirSync(dirName)) {
      if (dirEntry.isDirectory) {
        readDir(`${dirName}/${dirEntry.name}`);
      } else if (dirEntry.isFile && dirEntry.name.endsWith(".bs")) {
        run(`${dirName}/${dirEntry.name}`);
      }
    }
  }
}
