# beersensory

A Rust library and CLI tool to assist with beer sensory analysis. Use it to identify, categorize, and record beer flavors and aromas.

## Sensory Categories

### 1. Aroma & Flavor Categories

- **Malt**
    - Biscuit, Bread, Caramel, Chocolate, Coffee, Toast, Toffee, Nutty, Roasted
- **Hops**
    - Citrus, Pine, Floral, Herbal, Spicy, Earthy, Resinous, Tropical, Stone Fruit
- **Yeast**
    - Fruity (Esters): Banana, Pear, Apple, Apricot
    - Phenolic: Clove, Pepper, Smoke, Medicinal
- **Other**
    - DMS (Cooked Corn), Diacetyl (Butter), Acetaldehyde (Green Apple), Sulfur, Metallic

### 2. Mouthfeel

- Body (Light, Medium, Full)
- Carbonation (Low, Medium, High)
- Astringency
- Creaminess
- Warming (Alcohol)

### 3. Off-Flavors

- Oxidation (Stale, Cardboard, Sherry)
- Skunky (Lightstruck)
- Sour/Acidic (Lactic, Acetic)
- Solvent-like
- Musty

## How to Build

1. **Install Rust and Cargo**  
   If you haven't already, install Rust and Cargo:  
   [https://rustup.rs/](https://rustup.rs/)

2. **Clone the Repository**
   ```
   git clone https://github.com/yourusername/beersensory.git
   cd beersensory
   ```

3. **Build the Library**
   ```
   cargo build --release
   ```

4. **Run the CLI Tool**
   ```
   cargo run -- [options]
   ```

## Usage

Import the library in your Rust project or use the CLI to analyze and record sensory notes.

## Contributing

Contributions are welcome! Please open issues or pull requests.

## License

MIT