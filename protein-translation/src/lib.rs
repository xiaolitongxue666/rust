pub fn translate(rna: &str) -> Option<Vec<&str>> {
    if rna.is_empty() {
        return Some(vec![]);
    }

    let mut proteins = Vec::new();
    let bytes = rna.as_bytes();

    for chunk in bytes.chunks(3) {
        if chunk.len() < 3 {
            return None;
        }

        let codon = std::str::from_utf8(chunk).ok()?;

        match codon {
            "UAA" | "UAG" | "UGA" => break,
            "AUG" => proteins.push("Methionine"),
            "UUU" | "UUC" => proteins.push("Phenylalanine"),
            "UUA" | "UUG" => proteins.push("Leucine"),
            "UCU" | "UCC" | "UCA" | "UCG" => proteins.push("Serine"),
            "UAU" | "UAC" => proteins.push("Tyrosine"),
            "UGU" | "UGC" => proteins.push("Cysteine"),
            "UGG" => proteins.push("Tryptophan"),
            _ => return None,
        }
    }

    Some(proteins)
}
