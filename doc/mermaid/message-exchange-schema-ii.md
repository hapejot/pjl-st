# Message Exchange Schema II

## Original: Abb. 7.3 Schema eines Botschaftenaustausches II
**Translation**: Figure 7.3 Schema of a Message Exchange II

## German Terms Translation
- **OBJEKTKONTUR** → Object Context
- **METHODENVERZEICHNIS** → Method Directory
- **SELEKTOR** → Selector
- **METHODE** → Method
- **QUELLTEXT** → Source Text
- **BOTSCHAFTSKONTUR** → Message Context
- **TEMPORÄREBEREICH** → Temporary Area
- **AUSWERTUNGSBEREICH** → Execution Area
- **CODESEGMENT** → Code Segment
- **Steuerangaben** → Control Information
- **LITERALBEREICH** → Literal Area
- **PROGRAMMFRAGMENT** → Program Fragment
- **zugehörige Klasse zu Exemplar** → "associated class to instance"
- **durch Methodenbestimmung konstruiert** → "constructed through method determination"

## Mermaid Diagram

```mermaid
graph TB
    %% Object Context and Class Hierarchy
    OC1["Object Context<br/>EP ┃ MDP"]
    OC1 -->|"Superclass"| SuperClass["Superclass"]
    
    %% Associated Class
    OC2["Object Context<br/>EP ┃ MDP<br/>Associated Class<br/>to Instance"]
    
    %% Method Directories
    OC1 --> MD1["Method Directory<br/>Selector ┃ Method ┃ Source Text"]
    OC2 --> MD2["Method Directory<br/>Selector ┃ Method ┃ Source Text"]
    
    %% Symbol Connection
    SYMBOL["Symbol"] --> MD2
    
    %% Target Object Context
    OC3["Object Context c<br/>EP"]
    
    %% Message Context (constructed through method determination)
    OC3 --> MC["Message Context b<br/>EP ┃ RP ┃ CP ┃ IP ┃ SP<br/>Temporary Area<br/>Execution Area"]
    
    %% Code Segment
    MC --> CS["Code Segment d<br/>Control Information<br/>Literal Area"]
    
    %% Program Fragment
    MD2 --> PF["Program Fragment"]
    PF --> CS
    
    %% Method determination flow
    MD1 -.->|"Method Lookup"| MD2
    MD2 -.->|"Constructs via Method Determination"| MC
    
    %% Styling
    classDef objectContext fill:#e3f2fd,stroke:#1976d2,stroke-width:2px
    classDef messageContext fill:#fff3e0,stroke:#f57c00,stroke-width:2px
    classDef method fill:#fce4ec,stroke:#c2185b,stroke-width:2px
    classDef code fill:#f3e5f5,stroke:#7b1fa2,stroke-width:2px
    classDef symbol fill:#e8f5e8,stroke:#388e3c,stroke-width:2px
    
    class OC1,OC2,OC3,SuperClass objectContext
    class MC messageContext
    class MD1,MD2,PF method
    class CS code
    class SYMBOL symbol
```

## Sequence View

```mermaid
sequenceDiagram
    participant OC1 as Object Context 1
    participant MD1 as Method Directory 1  
    participant OC2 as Object Context 2
    participant MD2 as Method Directory 2
    participant MC as Message Context b
    participant CS as Code Segment d
    
    Note over OC1, CS: Message Exchange Process
    
    OC1->>MD1: Lookup method
    MD1->>MD2: Delegate to associated class
    MD2->>MC: Construct message context
    Note over MC: Method determination<br/>creates execution context
    MC->>CS: Load code segment
    CS->>MC: Execute with control info
    MC->>OC1: Return result
```