# Metal Sniffer

Metal Sniffer is a lightweight realtime packet sniffer built using [Tauri](https://tauri.app/) and written in Rust. This application is designed to help you monitor network traffic in a user-friendly way. It is packaged as a macOS `.dmg` file and is available for download.

---

## Features

- **Cross-platform foundation:** Built with Tauri, Metal Sniffer is optimized for macOS and leverages the power of Rust for performance.
- **Packet inspection:** View detailed network traffic information in real-time.
- **Filtering capabilities:** Apply custom filters to capture only the packets that match specific criteria, improving the focus and efficiency of network analysis.
- **Minimalistic and lightweight:** Runs smoothly with a small footprint.

---

## How Metal Sniffer Works

Metal Sniffer leverages the Rust programming language for its robust and memory-safe features, ensuring high performance and reliability during network traffic monitoring.

### Packet Sniffing

The application uses Rust libraries to access and process raw network data, capturing packets from the specified network interface. It provides detailed insights into each packet, including source and destination addresses, protocols, and payloads.

### Filtering Capabilities

Users can define filters based on parameters such as IP addresses, port numbers, or specific protocols. These filters are implemented using Rust's powerful pattern matching and efficient data processing capabilities, allowing the app to quickly isolate and display relevant packets while discarding irrelevant traffic.

This filtering mechanism enhances the usability of Metal Sniffer by enabling targeted network analysis, making it an invaluable tool for developers, network administrators, and cybersecurity professionals.

### Packet Grouping

A key feature of Metal Sniffer is its ability to automatically group packets by matching source and destination IP addresses. This allows users to quickly identify patterns in network traffic and focus on specific communication flows. Each group can be expanded to reveal detailed information about individual packets within the group, providing both a high-level overview and granular insights into the traffic.

---

## Download and Installation

### System Requirements

- **macOS:** Version 10.15 (Catalina) or newer

### Installation Steps

1. **Download the App**

   - [Download Metal Sniffer (DMG)]([https://github.com/your-repo/MetalSniffer/releases/latest](https://github.com/jwaguirr/MetalSniffer/releases/tag/app-v0.1.0))

2. **Run the App**

   - After downloading the `.dmg` file, double-click it to open.
   - Drag the `Metal Sniffer` app into your `Applications` folder.

3. **Grant Permissions** *(if needed)*

   - macOS may flag the app as "damaged" because it is not signed with an Apple Developer ID.
   - To bypass this:
     ```bash
     xattr -cr /path/to/Metal\ Sniffer.app
     ```
     Replace `/path/to/Metal\ Sniffer.app` with the actual path to the app.

4. Open the app and start sniffing packets!

---

## Notes for Users

- This is my **first miniature Rust project**, created to deepen my familiarity with the language. Metal Sniffer serves as both a practical tool and a learning experience.
- If you encounter any issues, feel free to open a [GitHub issue](https://github.com/jwaguirr/MetalSniffer/issues).

---

## Future Plans

- Add support for Windows and Linux platforms.
- Introduce advanced filtering capabilities for specific packet types.
- Enhance the UI for an even more user-friendly experience.
- I want to enhance application-layer recognition by adding support for identifying more protocols based on port numbers and traffic patterns.
- I would also like to decode the payload to identify cases where parts are UTF-8 encoded.

---

## Contributing

Contributions are welcome! If you have ideas for improvements or bug fixes, feel free to fork the repository and submit a pull request.

---

## License

Metal Sniffer is released under the [MIT License](https://opensource.org/licenses/MIT).

---

## Contact

For any questions or suggestions, reach out to me on GitHub or open an issue in the repository.

