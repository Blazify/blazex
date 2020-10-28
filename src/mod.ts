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
 * @returns Object of parsed, tokens and errors as keys. Note all keys are optional
 * @description Runs a BlazeScript Code
 */
export function run(
  name: string,
  code?: string,
): { parsed?: BinOpNode | NumberNode; tokens?: Token[]; errors?: Err[] } {
  // Tokenizing and Lexing the code
  const { tokens, errors } = new Lexer(
    name,
    code ?? new TextDecoder("utf-8").decode(Deno.readFileSync(Deno.args[0])),
  ).makeTokens();
  // if errors then return without parsing
  if (errors) {
    console.log(errors.map((error) => error.formatted()).join(", \n"));
    return { tokens, errors };
  }
  // no errors in lexing so now parsing
   const { node, error } = new Parser(tokens!).parse();
   if(error) console.error(error.formatted())
   else if(node) {
     const interpreter = new Interpreter();
     console.log(interpreter.visit(node).represent())
   }
   // if(tokens) tokens.forEach(token => console.log(token.represent()));
  // return tokens and parsed binary op nodes
  return { tokens };
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
