import java.util.*;
import java.io.*;

public class Quickscope {
    static class Declaration {
        byte[] type;
        int scope;
        Declaration shadow;

        Declaration(byte[] type, int scope, Declaration shadow) {
            this.type = type;
            this.scope = scope;
            this.shadow = shadow;
        }
    }

    static HashSet<Integer> scopes = new HashSet<>();
    static Stack<Integer> scopesStack = new Stack<>();
    static int curScope = 0;
    static HashMap<Integer, Declaration> declarations = new HashMap<>();
    static StringBuilder output = new StringBuilder();

    public static void main(String args[]) throws IOException {
        int lines = readNum();
        byte[][] line;
        scopes.add(curScope);

        try {
            for (int lineNo = 0; lineNo < lines; lineNo++) {
                line = readLine();
                parse(line, lineNo);
            }
        } catch (EndException e) {
            // Ugly exception control-flow >_>
            // All because I decided to do ugly global state programming...
        }
        System.out.print(output);
    }

    static int readNum() throws java.io.IOException {
        int num = 0, b;
        while ((b = System.in.read()) != '\n' && b != ' ' && b != -1)
            num = num * 10 + b - '0';
        return num;
    }

    static byte[][] readLine() throws IOException {
        int b, j = 0;
        byte[][] line = new byte[3][7];

        do {
            int i = 0;
            byte[] bytes = new byte[7];

            do {
                b = System.in.read();
                if (b == ' ' || b == '\n')
                    break;
                bytes[i++] = (byte) b;
            } while (b > 64);

            line[j++] = bytes;
        } while (b > 31);

        return line;
    }

    static void parse(byte[][] line, int lineNo) throws IOException {
        switch (line[0][0]) {
            case '{':
                enter(lineNo);
                break;
            case '}':
                leave(curScope);
                break;
            case 'D':
                declare(line[1], line[2]);
                break;
            case 'T':
                typeof(line[1]);
                break;
            default:
                throw new AssertionError("Invalid input");
        }
    }

    static void declare(byte[] ident, byte[] type) throws IOException {
        var hash = Arrays.hashCode(ident);
        var decl = declarations.get(hash);

        if (decl != null && decl.scope == curScope) {
            output.append("MULTIPLE DECLARATION\n");
            throw new EndException();
        }

        declarations.put(hash, new Declaration(type, curScope, decl));
    }

    static void typeof(byte[] ident) throws IOException {
        var hash = Arrays.hashCode(ident);
        var decl = declarations.get(hash);

        while (decl != null && !scopes.contains(decl.scope)) {
            decl = decl.shadow;
        }

        if (decl == null) {
            output.append("UNDECLARED\n");
        } else {
            output.append(new String(decl.type));
            output.append('\n');
        }

        declarations.put(hash, decl);
    }

    static void enter(int scope) {
        scopes.add(scope);
        scopesStack.push(curScope);
        curScope = scope;
    }

    static void leave(int scope) {
        scopes.remove(scope);
        curScope = scopesStack.pop();
    }
}

class EndException extends RuntimeException {
}
