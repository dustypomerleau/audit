# This file is basically everything in `auth_setup.edgeql` with the exception of secrets.
# To run the setup script, pipe it into edgedb:
# `cat my_script.edgeql | edgedb`

# 14 days allowed session time
configure current branch set
ext::auth::AuthConfig::token_time_to_live := <duration>"336 hours";

# todo: change these to the actual values you'll put into:
# `edgedb ui` > Auth > Providers > Login UI > redirect_to
# note: the value of redirect_to in the Auth UI is the address in your application
# where you want to end up after auth is complete.
# The redirect for the callback function within the auth flow has to be set in the GCP console.
configure current branch set
ext::auth::AuthConfig::allowed_redirect_urls := {
    "https://audit.viceye.au",
    "https://localhost:3000",
    "http://localhost:3000",
    "http://localhost:3000/code"
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
# Run `edgedb ui`, go to > Auth > signing key, and refresh it.
# To set it to a specific value, you can use a command like:
# 
# CONFIGURE CURRENT BRANCH SET
# ext::auth::AuthConfig::auth_signing_key := "<my-signing-key>";
# 
# but it's much easier to let EdgeDB generate this.

# You will also need to enable the built-in UI at:
# > Auth > Providers > Enable UI
# and set the redirect URLs.
# Full URLs are needed, including protocol, as you are basically coming from another site before the redirect.
