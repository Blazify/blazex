import { Interpreter } from "./core/interpreter.ts";
import { Lexer } from "./core/lexer.ts";
import { BinOpNode } from "./core/node/binary_op_node.ts";
import { NumberNode } from "./core/node/number_nodes.ts";
import { Parser } from "./core/parser/parser.ts";
import { Token } from "./core/token.ts";
import { Err } from "./error/err.ts";

/**
 * @param name File name, reads code from the file if code not provided
 * @param code Optional code to evaluate
 * @description Runs a BlazeScript Code
 */
export function run(
  name: string,
  code?: string,
) {
  // Tokenizing and Lexing the code
  const { tokens, errors } = new Lexer(
    name,
    code ?? new TextDecoder("utf-8").decode(Deno.readFileSync(Deno.args[0])),
  ).makeTokens();
  // if errors then return without parsing
  if (errors) {
    console.log(errors.map((error) => error.formatted()).join(", \n"));
    return { errors };
  }
  // no errors in lexing so now parsing
  const { node, error } = new Parser(tokens!).parse();
  const interpreter = new Interpreter();
  const { value, error: rterr } = interpreter.visit(node!);
  console.log("Interpreted Output:", value?.represent() ?? null, "\nParse Error:", error?.formatted() ?? null ,"\nRuntime Error:", rterr?.formatted() ?? null)
  // if(tokens) tokens.forEach(token => console.log(token.represent()));
  // return tokens and parsed binary op nodes
  return { tokens, interpreted: interpreter.visit(node!) };
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
