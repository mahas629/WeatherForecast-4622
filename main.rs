Тут нижче наведено приклад програми Rust, яка виконує базову обробку даних. Ця програма зчитує вміст вхідного текстового файлу, обробляє дані та записує результати у вихідний файл.

```rust
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;

// Функція для обробки рядків
fn process_line(line: &str) -> io::Result<String> {
    let processed_line = line.to_uppercase();  // Приклад обробки: перетворення в верхній регістр
    Ok(processed_line)
}

fn main() -> io::Result<()> {
    // Відкриття вхідного файлу
    let input_path = Path::new("input.txt");
    let input_file = File::open(&input_path)?;
    let reader = io::BufReader::new(input_file);

    // Створення вихідного файлу
    let output_path = Path::new("output.txt");
    let output_file = File::create(&output_path)?;
    let mut writer = BufWriter::new(output_file);

    // Читання рядків з вхідного файлу
    for line in reader.lines() {
        let line = line?;
        
        // Обробка рядка
        let processed_line = process_line(&line)?;

        // Запис обробленого рядка в вихідний файл
        writer.write_all(processed_line.as_bytes())?;
        writer.write_all(b"\n")?;
    }

    // Закриття вихідного файлу
    writer.flush()?;

    Ok(())
}
```

Ця програма є простим прикладом того, як можна використовувати Rust для читання та запису файлів, а також для обробки рядків. В залежності від ваших потреб, ви можете змінити функцію `process_line` так, щоб вона виконувала потрібну вам обробку даних.
