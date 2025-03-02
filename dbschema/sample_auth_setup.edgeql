# This file is basically everything in `auth_setup.edgeql` with the exception of secrets.
# 
# To run the setup script, pipe it into gel:
# `cat my_script.edgeql | gel`

# 14 days allowed session time
configure current branch set
ext::auth::AuthConfig::token_time_to_live := <duration>"336 hours";

# note: the value of redirect_to in the Auth UI is the address in your application
# where you want to end up after auth is complete.
# 
# The redirect for the callback function _within_ the auth flow has to be set in the GCP console.
configure current branch set
ext::auth::AuthConfig::allowed_redirect_urls := {
    "http://localhost:3000",
    "http://localhost:3000/code",
    "https://audit.viceye.au",
    "https://audit.viceye.au/code",
};

configure current branch set
ext::auth::AuthConfig::app_name := { "Vic Eye Cataract Audit" };

configure current branch set
ext::auth::AuthConfig::brand_color := { "#ff7b00" };

configure current branch
insert ext::auth::GoogleOAuthProvider {
    secret := "<my-secret-here>",
    client_id := "<my-client-id-here>",
};

# After running this script, you still need to generate an auth signing key.
# Run `gel ui`, go to > Auth > Config > Auth signing key, and refresh it.
# 
# To set it to a specific value, you can use a command like:
# 
# CONFIGURE CURRENT BRANCH SET
# ext::auth::AuthConfig::auth_signing_key := "<my-signing-key>";
# 
# but it's much easier to let Gel generate this.

# You will also need to enable the built-in UI at:
# > Auth > Providers/UI > Built-in Login UI
# and set the redirect URLs.
# Full URLs are needed, including protocol, as you are basically coming from another site before the redirect.
# "http://localhost:3000/code" in dev
# "https://audit.viceye.au/code" in prod
