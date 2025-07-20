# 🤝 Contributing to RustyConfig

Vielen Dank für dein Interesse an RustyConfig! Wir freuen uns über alle Beiträge, die das Projekt verbessern.

## 📋 Inhaltsverzeichnis

- [Code of Conduct](#code-of-conduct)
- [Wie kann ich beitragen?](#wie-kann-ich-beitragen)
- [Entwicklungsumgebung einrichten](#entwicklungsumgebung-einrichten)
- [Pull Request Prozess](#pull-request-prozess)
- [Coding Standards](#coding-standards)
- [Tests](#tests)
- [Dokumentation](#dokumentation)
- [Release Prozess](#release-prozess)

## 📜 Code of Conduct

Dieses Projekt folgt dem [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). Durch die Teilnahme erwartet man, dass dieser Code eingehalten wird.

## 🚀 Wie kann ich beitragen?

### Arten von Beiträgen

- 🐛 **Bug Reports**: Melde Fehler und Probleme
- ✨ **Feature Requests**: Schlage neue Features vor
- 🔧 **Code Contributions**: Implementiere Features oder Fixes
- 📚 **Documentation**: Verbessere die Dokumentation
- 🧪 **Tests**: Erweitere die Testabdeckung
- 🌍 **Translations**: Übersetze Dokumentation

### Erste Schritte

1. Schaue dir die [Issues](https://github.com/yourusername/rusty-config/issues) an
2. Suche nach Issues mit dem Label "good first issue" oder "help wanted"
3. Kommentiere auf einem Issue, dass du daran arbeiten möchtest
4. Forke das Repository und erstelle einen Feature-Branch

## 🛠️ Entwicklungsumgebung einrichten

### Voraussetzungen

- Rust 1.70 oder höher
- Cargo
- Git

### Setup

```bash
# Repository klonen
git clone https://github.com/yourusername/rusty-config.git
cd rusty-config

# Dependencies installieren
cargo build

# Tests ausführen
cargo test

# Linting
cargo clippy

# Formatierung
cargo fmt
```

### Nützliche Tools

```bash
# Rustup Komponenten installieren
rustup component add rustfmt
rustup component add clippy
rustup component add rust-docs

# Cargo Tools installieren
cargo install cargo-audit
cargo install cargo-tarpaulin  # für Code Coverage
cargo install cargo-watch       # für automatisches Neustarten
```

## 🔄 Pull Request Prozess

### 1. Branch erstellen

```bash
git checkout -b feature/amazing-feature
# oder
git checkout -b fix/bug-description
```

### 2. Änderungen entwickeln

- Schreibe sauberen, dokumentierten Code
- Füge Tests für neue Features hinzu
- Stelle sicher, dass alle Tests bestehen
- Aktualisiere die Dokumentation

### 3. Commits erstellen

```bash
# Änderungen stagen
git add .

# Commit mit aussagekräftiger Nachricht
git commit -m "feat: add new validation rule for email addresses

- Add TypeValidator::email method
- Add tests for email validation
- Update documentation with examples"
```

### 4. Push und Pull Request

```bash
git push origin feature/amazing-feature
```

Erstelle dann einen Pull Request auf GitHub mit:

- **Titel**: Kurze Beschreibung der Änderung
- **Beschreibung**: Detaillierte Erklärung der Änderungen
- **Checklist**: Alle relevanten Punkte abhaken

### 5. Pull Request Template

```markdown
## 📝 Beschreibung

Kurze Beschreibung der Änderungen.

## 🔗 Related Issues

Fixes #123
Closes #456

## 🧪 Tests

- [ ] Unit Tests hinzugefügt/aktualisiert
- [ ] Integration Tests hinzugefügt/aktualisiert
- [ ] Alle Tests bestehen

## 📚 Dokumentation

- [ ] Code-Kommentare hinzugefügt/aktualisiert
- [ ] README aktualisiert
- [ ] API-Dokumentation aktualisiert

## ✅ Checklist

- [ ] Code folgt den Coding Standards
- [ ] Tests hinzugefügt und bestehen
- [ ] Dokumentation aktualisiert
- [ ] Keine Breaking Changes (oder dokumentiert)
- [ ] Commit-Nachrichten folgen Conventional Commits
```

## 📏 Coding Standards

### Rust Code Style

- Verwende `rustfmt` für Formatierung
- Folge den `clippy` Warnungen
- Verwende aussagekräftige Variablen- und Funktionsnamen
- Schreibe Dokumentation für öffentliche APIs

### Code-Beispiel

```rust
/// Validiert eine E-Mail-Adresse.
///
/// # Arguments
///
/// * `email` - Die zu validierende E-Mail-Adresse
/// * `field_name` - Der Name des Feldes für Fehlermeldungen
///
/// # Returns
///
/// `ConfigResult<()>` - Ok(()) wenn gültig, Fehler wenn ungültig
///
/// # Examples
///
/// ```
/// use rusty_config::validator::TypeValidator;
///
/// let result = TypeValidator::email("test@example.com", "email");
/// assert!(result.is_ok());
/// ```
pub fn email(email: &str, field_name: &str) -> ConfigResult<()> {
    if !email.contains('@') || !email.contains('.') {
        return Err(ConfigError::Validation(format!(
            "Feld '{}' muss eine gültige E-Mail-Adresse sein",
            field_name
        )));
    }
    Ok(())
}
```

### Commit-Nachrichten

Verwende [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add new validation rule for email addresses
fix: resolve issue with hot-reload on Windows
docs: update README with new examples
test: add comprehensive tests for TypeValidator
refactor: improve error handling in ConfigBuilder
```

## 🧪 Tests

### Test-Struktur

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_valid_email() {
        let result = TypeValidator::email("test@example.com", "email");
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_invalid_email() {
        let result = TypeValidator::email("invalid-email", "email");
        assert!(result.is_err());
    }
}
```

### Test-Ausführung

```bash
# Alle Tests
cargo test

# Spezifische Tests
cargo test test_valid_email

# Tests mit Output
cargo test -- --nocapture

# Code Coverage
cargo tarpaulin
```

## 📚 Dokumentation

### Code-Dokumentation

- Dokumentiere alle öffentlichen APIs
- Verwende Beispiele in der Dokumentation
- Halte die Dokumentation aktuell

### README Updates

- Aktualisiere die README bei neuen Features
- Füge Beispiele hinzu
- Aktualisiere die Installation-Anweisungen

## 🚀 Release Prozess

### Versionierung

Wir verwenden [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking Changes
- **MINOR**: Neue Features (rückwärtskompatibel)
- **PATCH**: Bug Fixes (rückwärtskompatibel)

### Release Checklist

- [ ] Alle Tests bestehen
- [ ] Dokumentation ist aktuell
- [ ] CHANGELOG ist aktualisiert
- [ ] Version in Cargo.toml ist erhöht
- [ ] Release Notes sind geschrieben

## 🆘 Hilfe

### Wo bekomme ich Hilfe?

- [Issues](https://github.com/yourusername/rusty-config/issues) - Für Bug Reports und Feature Requests
- [Discussions](https://github.com/yourusername/rusty-config/discussions) - Für Fragen und Diskussionen
- [Documentation](https://docs.rs/rusty-config) - API-Dokumentation

### Häufige Fragen

**Q: Wie kann ich ein neues Feature vorschlagen?**
A: Erstelle ein Issue mit dem Label "enhancement" und beschreibe das Feature detailliert.

**Q: Wie kann ich einen Bug melden?**
A: Erstelle ein Issue mit dem Label "bug" und füge alle relevanten Informationen hinzu.

**Q: Wie kann ich bei der Dokumentation helfen?**
A: Forke das Repository und erstelle Pull Requests für Dokumentationsverbesserungen.

## 🙏 Danksagung

Vielen Dank für deine Beiträge zu RustyConfig! Jeder Beitrag, egal wie klein, hilft dabei, das Projekt zu verbessern.

---

**Entwickelt mit ❤️ von der Rust-Community** 