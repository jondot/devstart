use copypasta::ClipboardContext;
use copypasta::ClipboardProvider;
use eyre::eyre;
use eyre::Result;
use owo_colors::OwoColorize;

/// Copy to clipboard
///
/// # Errors
///
/// This function will return an error if clipboard API fails
pub fn copy(text: &str) -> Result<()> {
    let mut ctx = ClipboardContext::new().map_err(|_| eyre!("cannot get clipboard"))?;
    ctx.set_contents(text.to_string())
        .map_err(|_| eyre!("cannot set clipboard content"))?;
    println!("copied `{}`", text.green());
    Ok(())
}
