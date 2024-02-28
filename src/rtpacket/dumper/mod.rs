use crate::base::Layer;
use hex::encode as hex_encode;

/// `Dumper` trait allows types to dump verbose information about themselves.
///
/// Types implementing `Dumper` can provide a detailed string representation
/// of their state, which can be included in their output for debugging or
/// logging purposes.
pub(crate) trait Dumper {
    /// Returns a verbose string representation of the implementing type.
    fn dump(&self) -> String;
}

/// Outputs a very verbose string representation of a layer.
/// Concatenates the layer's string representation with a hex dump of its contents.
pub fn layer_dump<L: Layer + ?Sized>(layer: &L) -> String where L: Dumper {
    let mut result = String::new();

    // Layer's string representation
    writeln!(result, "{}", layer.layer_string()).unwrap();

    // Check if the layer implements Dumper and append its dump
    let dump = layer.dump();
    if !dump.is_empty() {
        writeln!(result, "{}", dump).unwrap(); // Ensure newline at the end of the dump
    }

    // Append hex dump of the layer's contents
    let hex_dump = hex_encode(layer.layer_contents());
    writeln!(result, "{}", hex_dump).unwrap();

    result
}
