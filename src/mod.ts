import { Context } from "./core/context.ts";
import { Interpreter } from "./core/interpreter.ts";
import { Lexer } from "./core/lexer.ts";
import { Parser } from "./core/parser/parser.ts";
import { Position } from "./error/position.ts";
import { RuntimeError } from "./error/runtimeerr.ts";

/**
 * @param name File name, reads code from the file if code not provided
 * @param code Optional code to evaluate
 * @description Runs a BlazeScript Code
 */
export function run(
  name: string,
  code?: string,
) {
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
      return console.log(error.formatted());
    }
    const interpreter = new Interpreter();
    const context = new Context("<Global>");
    const { value, error: rterr } = interpreter.visit(node!, context);
    console.log(
      "Interpreted Output:",
      value?.represent() ?? null,
      "\nRuntime Error:",
      rterr?.formatted() ?? null,
    );
    return { tokens, interpreted: interpreter.visit(node!, context) };
  } catch (e) {
    console.log(
      "Unexpected Error Given by Deno.\nPlease open a issue at https://github.com/RoMeAh/blazescript/issues/ with description of\n" +
        e.stack,
    );
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
