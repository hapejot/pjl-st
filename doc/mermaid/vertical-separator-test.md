# Mermaid Vertical Separator Test

## Different Vertical Line Options

```mermaid
graph TB
    A["Heavy Vertical Bar<br/>EP ┃ RP ┃ CP"]
    B["Light Vertical<br/>EP │ RP │ CP"] 
    C["Double Vertical<br/>EP ‖ RP ‖ CP"]
    D["Broken Bar<br/>EP ¦ RP ¦ CP"]
    E["Bullet Points<br/>EP • RP • CP"]
    F["Colons<br/>EP : RP : CP"]
    G["Dashes<br/>EP - RP - CP"]
    H["Forward Slashes<br/>EP / RP / CP"]
    
    A --> B --> C --> D
    E --> F --> G --> H
    
    classDef default fill:#f9f9f9,stroke:#333,stroke-width:2px
```

## Recommended: Heavy Vertical Bar (┃)

```mermaid
graph TB
    OC["Object Context<br/>EP ┃ MDP ┃ Status"]
    MC["Message Context<br/>EP ┃ RP ┃ CP ┃ IP ┃ SP<br/>Temporary Area<br/>Execution Area"]
    MD["Method Directory<br/>Selector ┃ Method ┃ Source Text"]
    
    OC --> MC
    MC --> MD
    
    classDef default fill:#e3f2fd,stroke:#1976d2,stroke-width:2px
```

## How to Type These Characters

### Windows:
- **Heavy Vertical Bar (┃)**: Alt + 9475 (on numpad)
- **Light Vertical (│)**: Alt + 179 (on numpad)

### Mac:
- **Use Character Viewer**: Cmd + Ctrl + Space, search "vertical"

### Linux:
- **Compose key**: Compose + | + |
- **Unicode input**: Ctrl + Shift + U, then 2503

### Any Platform:
- **Copy from here**: ┃ │ ‖ ¦
- **HTML entities**: `&#x2503;` for ┃