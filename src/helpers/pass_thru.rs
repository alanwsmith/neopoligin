use minijinja::Error;

// this is a pass thrugh that just returns what's
// sent to it. The purpose is to allow minijinja
// to then escape the full set of content for
// view

pub fn pass_thru(html: String) -> Result<String, Error> {
    Ok(html)
}
