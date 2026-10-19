# Monkey Lang

A Monkey programming language implementation written in Rust.

## Installation

```bash
cargo install monkeyc
```

```mermaid
flowchart TD
    A["Source Code<br/>(main.monkey)"] --> B["Lexer<br/>lexer.rs"]
    B -->|"Token stream"| C["Parser<br/>parser.rs"]
    C -->|"AST (Program)"| D["Compiler<br/>compiler.rs"]
    D -->|"Bytecode<br/>bytecode.rs"| E["Virtual Machine<br/>vm.rs"]
    E -->|"Result"| F["Output"]

    G["Token definitions<br/>token.rs"] -.-> B
    G -.-> C
    H["REPL<br/>repl.rs"] --> B
    I["File loader<br/>file.rs"] --> B

    style A fill:#00000F
    style F fill:#d4edda
    style E fill:#1113cd
    style D fill:#f8d7da
    style C fill:#e2d5f1
    style B fill:#d1ecf1
```

# Stage-By-Stage Details

```mermaid
flowchart LR
    subgraph Frontend["Frontend (Interpretation-ish)"]
        direction TB
        SRC["Raw text<br/>input source"]
        LEX["Lexer<br/>─────────<br/>• read_char()<br/>• next_token()<br/>• skip_whitespace()"]
        TOK["Tokens<br/>IDENT, INT, PLUS,<br/>LET, FN, IF, ..."]
        PAR["Parser<br/>─────────<br/>• parse_program()<br/>• Pratt parsing<br/>• precedence"]
        AST["AST<br/>Program → Statements<br/>LetStmt, ExprStmt,<br/>InfixExpr, ..."]

        SRC --> LEX --> TOK --> PAR --> AST
    end

    subgraph Backend["Backend (Compilation)"]
        direction TB
        COMP["Compiler<br/>─────────<br/>• walk AST<br/>• emit(opcode)<br/>• symbol table<br/>• scope tracking"]
        BC["Bytecode<br/>─────────<br/>Instructions<br/>Constants pool<br/>+ OpAdd, OpCall,<br/>OpGetGlobal, ..."]
        VM["VM<br/>─────────<br/>• fetch-decode-execute<br/>• stack machine<br/>• call frames"]

        COMP --> BC --> VM
    end

    AST --> COMP
    VM --> OUT["Result / Output"]
```

# Data Flow (Objects Passed Between Stages)

```mermaid
sequenceDiagram
    participant U as User
    participant F as file.rs
    participant L as lexer.rs
    participant P as parser.rs
    participant C as compiler.rs
    participant B as bytecode.rs
    participant V as vm.rs

    U->>F: cargo run main.monkey
    F->>L: source string
    loop until EOF
        L->>P: Token
    end
    P->>C: AST (Program)
    C->>B: emit(OpConstant, ...)
    C->>C: emit(OpAdd, ...)
    C->>B: Instructions
    B->>V: Bytecode (instructions + constants)
    V->>V: fetch → decode → execute
    V-->>U: println! result

```

# Module Map

```mermaid
graph TD
    MAIN["main.rs<br/>(entry point)"]

    MAIN --> FILE["file.rs<br/>read source file"]
    MAIN --> REPL["repl.rs<br/>interactive mode"]

    FILE --> LEX["lexer.rs"]
    REPL --> LEX

    LEX --> TOKEN["token.rs<br/>token types"]
    LEX --> PARSER["parser.rs"]

    PARSER --> ASTNODE["AST nodes<br/>(in parser.rs)"]
    PARSER --> COMPILER["compiler.rs"]

    COMPILER --> BYTECODE["bytecode.rs<br/>opcodes + Instructions"]
    COMPILER --> VM["vm.rs"]

    VM --> OUT["Output"]

    style MAIN fill:#111111
    style VM fill:#bbbbbb
    style OUT fill:#111111
```

# Instruction Cycle (VM Internals)

```mermaid

stateDiagram-v2
    [*] --> Fetch
    Fetch --> Decode: read instruction at ip
    Decode --> Execute: match opcode
    Execute --> Fetch: advance ip
    Execute --> Halt: OpHalt / end of code
    Halt --> [*]
```
