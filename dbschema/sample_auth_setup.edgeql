# This file is basically everything in `auth_setup.edgeql` with the exception of secrets.
# 
# To run the setup script, pipe it into gel:
# `cat my_script.edgeql | gel`

# 14 days allowed session time
configure current branch set
ext::auth::AuthConfig::token_time_to_live := <duration>"336 hours";

# For the avoidance of any future doubt:
# 
# 1. The `BASE_AUTH_URL` environment variable should be set according to the value in the Gel UI under 'Built-in UI sign-in URL' but minus everything after the word `auth`.
# 2. In the Gel UI, the `Config > Allowed redirect URLs` just needs to be set to the root URL for the app (like https://myapp.com + https://www.myapp.com). 
# 3. In the Gel UI, the `Providers/UI > Built-in Login UI > Redirect to` needs to be set to http://myapp.com/code.
# 4. In the Google OAuth config at https://console.cloud.google.com/auth/clients, the `authorised redirect URLs` need to be set to the values of `Gel UI > Auth > Config > OAuth callback endpoint`, but minus the :PORT component (except localhost, where you do need the port). For the OAuth client, you have to put the full callback URL (parent URLs won't match).
# 5. Include the full protocol ahead of the URL in all cases above.
#
configure current branch set
ext::auth::AuthConfig::allowed_redirect_urls := {
    "http://localhost:3000",
    "https://audit.viceye.au",
    "https://www.audit.viceye.au",
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
