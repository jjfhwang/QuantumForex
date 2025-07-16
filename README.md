# QuantumForex: Spectral Analysis & RL Optimized Forex Trading

Quantifying and exploiting Forex market microstructure anomalies through advanced signal processing and reinforcement learning.

This project, QuantumForex, is a sophisticated algorithmic trading system designed to identify and capitalize on subtle anomalies within Forex market microstructure. It leverages spectral analysis techniques to decompose time series data into its constituent frequencies, revealing hidden patterns and periodicities that are often obscured by traditional technical analysis methods. These patterns, indicative of short-term inefficiencies and order book dynamics, are then used to train reinforcement learning (RL) agents. The RL agents learn to generate optimal trading signals based on these spectral features, dynamically adapting to evolving market conditions. Furthermore, QuantumForex includes a robust backtesting framework, enabling rigorous evaluation of trading strategies against historical data, providing valuable insights into their performance and risk characteristics.

The core innovation lies in the synergistic combination of spectral analysis and reinforcement learning. By extracting meaningful spectral features, the RL agent is provided with a richer and more informative representation of the market state, leading to improved decision-making capabilities. The backtesting framework allows for the validation of these strategies under various market scenarios, ensuring robustness and mitigating the risk of overfitting. This comprehensive approach aims to move beyond simple technical indicators and exploit the complex, underlying dynamics of the Forex market to generate consistent and profitable trading signals. The entire system is built in Rust, guaranteeing performance and safety critical for real-time trading environments.

QuantumForex is not just a theoretical model; it's a practical tool for quantitative traders and researchers seeking to explore the frontiers of algorithmic Forex trading. The modular design of the project allows for easy customization and extension. Users can incorporate their own spectral analysis techniques, train different RL agents, or modify the backtesting framework to suit their specific needs. The project also provides a solid foundation for further research into the application of advanced machine learning methods in financial markets. This project provides a pathway towards more effective, adaptive, and robust automated trading strategies.

## Key Features

*   **Spectral Analysis Module:** Employs Fast Fourier Transform (FFT) and Wavelet Transform to decompose Forex time series data (e.g., bid-ask spreads, order book depth) into frequency components. Provides functions to extract key spectral features such as dominant frequencies, power spectral density, and spectral entropy.
*   **Reinforcement Learning Signal Generation:** Utilizes a Deep Q-Network (DQN) agent trained on historical market data and spectral features. The agent learns to generate buy, sell, or hold signals based on the observed market state and a defined reward function (e.g., Sharpe ratio, Profit/Loss).
*   **Customizable Reward Function:** Allows users to define their own reward functions based on various performance metrics, such as profit maximization, risk aversion, and drawdown minimization. The reward function is implemented as a Rust trait, enabling easy extension.
*   **Comprehensive Backtesting Framework:** Simulates trading strategies on historical data with realistic transaction costs and slippage. Includes detailed performance reporting, such as profit/loss curves, drawdown analysis, and Sharpe ratio calculation.
*   **Data Ingestion Pipeline:** Supports ingesting Forex data from various sources, including CSV files, API connections (e.g., OANDA, Interactive Brokers), and real-time market data feeds. Data is preprocessed and formatted for use in spectral analysis and RL training.
*   **Order Book Depth Analysis:** Module for processing and analyzing Level II order book data to gain deeper insights into market liquidity and order flow dynamics. This data is integrated into spectral analysis and RL training processes.
*   **High-Performance Rust Implementation:** Ensures speed and efficiency for real-time trading applications. Utilizes parallel processing and memory-efficient data structures for optimal performance.

## Technology Stack

*   **Rust:** The core programming language, chosen for its performance, safety, and concurrency features.
*   **ndarray:** A Rust library for numerical computation with multidimensional arrays, essential for spectral analysis and data manipulation.
*   **rustfft:** A high-performance Fast Fourier Transform (FFT) library for Rust, used for frequency domain analysis.
*   **rand:** A random number generator library for Rust, used for stochastic processes in RL and backtesting.
*   **serde:** A serialization and deserialization framework for Rust, used for handling configuration files and data persistence.
*   **tokio:** An asynchronous runtime for Rust, used for handling concurrent data streams and network communication.
*   **Polars:** A fast and efficient DataFrame library, optimized for data analysis and manipulation, used for handling Forex data.

## Installation

1.  **Install Rust:** Ensure you have Rust installed. The recommended way is to use `rustup`:
    `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

2.  **Clone the Repository:** Clone the QuantumForex repository from GitHub:
    `git clone https://github.com/jjfhwang/QuantumForex.git`
    `cd QuantumForex`

3.  **Build the Project:** Build the project using Cargo:
    `cargo build --release`

4.  **Install Dependencies:** Ensure you have any necessary system dependencies for data sources or plotting libraries. Specific requirements depend on your chosen data source and visualization methods.

## Configuration

QuantumForex uses a configuration file (e.g., `config.toml`) to specify various settings, such as data source parameters, RL agent hyperparameters, and backtesting parameters.

Example `config.toml`:

[data]
data_source = "csv"
csv_path = "data/EURUSD_H1.csv"
start_date = "2023-01-01"
end_date = "2023-12-31"

[rl]
learning_rate = 0.001
gamma = 0.99
epsilon = 0.1

[backtest]
initial_capital = 10000.0
transaction_cost = 0.0001
slippage = 0.0002

The project also uses environment variables. For example:

DATA_API_KEY= "YOUR_API_KEY"

These variables can be set in your `.env` file or directly in your shell.

## Usage

1.  **Data Ingestion:**
    Run the data ingestion module to load Forex data from your chosen source. The data is preprocessed and formatted for use in spectral analysis and RL training.

    Example command:
    `./target/release/quantumforex --data-ingest --config config.toml`

2.  **Spectral Analysis:**
    Perform spectral analysis on the ingested data to extract relevant features.

    Example command:
    `./target/release/quantumforex --spectral-analysis --config config.toml`

3.  **RL Training:**
    Train the reinforcement learning agent using the extracted spectral features and historical market data.

     Example command:
    `./target/release/quantumforex --rl-train --config config.toml`

4.  **Backtesting:**
    Evaluate the trained RL agent's performance using the backtesting framework.

     Example command:
    `./target/release/quantumforex --backtest --config config.toml`

The specific arguments and options for each module can be found by running the executable with the `--help` flag (e.g., `./target/release/quantumforex --data-ingest --help`). Detailed API documentation will be provided in a separate document.

## Contributing

We welcome contributions to QuantumForex! Please follow these guidelines:

*   Fork the repository.
*   Create a new branch for your feature or bug fix.
*   Write clear and concise commit messages.
*   Submit a pull request with a detailed description of your changes.
*   Ensure your code adheres to the Rust style guidelines (using `cargo fmt`).
*   Include unit tests for any new functionality.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/jjfhwang/QuantumForex/blob/main/LICENSE) file for details.

## Acknowledgements

We would like to acknowledge the contributions of the open-source community to the Rust ecosystem and the various libraries used in this project. Also, we would like to express our gratitude to individuals who provided valuable feedback and suggestions during the development of QuantumForex.