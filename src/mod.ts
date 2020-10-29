import { Context } from "./core/context.ts";
import { Interpreter } from "./core/interpreter.ts";
import { Lexer } from "./core/lexer.ts";
import { Number } from "./core/number.ts";
import { Parser } from "./core/parser/parser.ts";
import { RuntimeResult } from "./core/runtime_result.ts";
import { Err } from "./error/err.ts";
import { SymbolTable } from "./utils/symbol_table.ts";
/**
 * @param name File name, reads code from the file if code not provided
 * @param code Optional code to evaluate
 * @description Runs a BlazeScript Code
 */
export function run(
  name: string,
  code?: string,
): { interpreted?: RuntimeResult, error?: Err, errors?: Err[] } {
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
      console.log(error.formatted());
      return { error }
    }
    const interpreter = new Interpreter();
    const context = new Context("<Global>");
    const global = new SymbolTable();
    global.set("number", new Number(0))
    context.symbolTable = global;
    const { value, error: rterr } = interpreter.visit(node!, context);
    if(rterr) {
      console.log(rterr.formatted());
      return { error: rterr }
    }
    console.log(
      "Interpreted Output:",
      value?.represent() ?? null
    );
    return { interpreted: interpreter.visit(node!, context) };
  } catch (e) {
    throw "Unexpected Error Given by Deno.\nPlease open a issue at https://github.com/RoMeAh/blazescript/issues/ with description of\n" + (e.stack ?? e);
  }
}

if (!Deno.args[0]) {
  throw "No File or Flag provided";
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
