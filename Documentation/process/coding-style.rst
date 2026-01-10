# =============================================================================
# APRK OS - Coding Style Guide
# =============================================================================
# APRK OS Coding Style
# =============================================================================

## 1. Naming Conventions
- **Modules**: `snake_case` (e.g., `memory_manager`)
- **Types/Structs**: `PascalCase` (e.g., `ProcessDescriptor`)
- **Functions**: `snake_case` (e.g., `allocate_page`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `PAGE_SIZE`)
- **Macros**: `snake_case!` (e.g., `aprk_info!`)

## 2. Indentation & Formatting
- Use **4 spaces** for indentation.
- Maximum line length: **100 characters**.
- Use `cargo fmt` to maintain consistency.

## 3. Documentation & Comments
- Use `///` for public APIs.
- Use `//` for internal implementation details.
- Every `unsafe` block **MUST** have a `// SAFETY:` comment explaining why it is safe.

## 4. Error Handling
- Favor `Result<T, E>` over `panic!` in kernel code.
- Only use `panic!` for truly unrecoverable states (e.g., failed to initialize early UART).

## 5. Module Organization
- One core concept per module.
- Keep `unsafe` code isolated and wrapped in safe abstractions where possible.
