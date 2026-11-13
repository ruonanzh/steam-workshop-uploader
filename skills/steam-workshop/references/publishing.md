# Publishing results and connection failures

This skill reuses the local Steam client session. Steam must be running, online, and signed in. The account needs the game's license and permission to modify the target item. Authentication failures, account restrictions, and publishing permissions are different errors; do not present every failure as a request to sign in again.

Reuse the same Workshop ID for updates to a Mod. Keep its ID, App ID, a verifiable publishing-account identifier, publishing directory, and latest result inside that Mod. Honor a newly specified target or explicit creation request. Reconcile conflicting records before modifying an item.

Visibility values are private, friends-only, unlisted, and public. An explicit request to publish publicly authorizes that action without another confirmation. For unspecified visibility, create privately and preserve existing visibility during updates. Use set-text for language changes without changing global visibility. Complete the requested localizations and presentation before a final public update when appropriate.

For mainland China network failures, identify the failing stage: SDK initialization, query, creation, content upload, preview upload, final submission, or community page access. A browser failure does not prove the upload failed, and a successful query does not prove uploads work. Use the existing Steam network environment. HTTP_PROXY, Git proxies, and browser proxies do not demonstrate coverage of Steam's full communication path. Report the actual stage and actionable next step; do not automatically change system proxies or repeatedly request passwords.

Read-only failures may receive bounded retries. After an uncertain creation, submission, or gallery append, check the ID, workshop.toml, gallery filenames, and actual text before replaying anything. Preserve records and report an unknown result when it cannot be reconciled. Some generic create/update waits lack an overall deadline; use the host execution tool's bounded waits, logs, and cancellation. Stopping a local process does not roll back the remote operation.

needs_workshop_agreement=true requires the player to handle terms in Steam. Do not claim the player has accepted them. Read back visibility; use an unauthenticated query or browser check when public availability matters.

In local tests, independent SDK processes did not launch the game executable, but Steam marked the App ID as running and performed cloud-sync checks. If the player already has the game open, do not close or restart it just for publishing. Keep Steam credentials, Guard codes, tokens, and account configuration out of both the skill package and upload directory.

Publishing, updating, localized text, and galleries have been tested on macOS Apple Silicon. Report Windows, Intel Mac, and different mainland China network results separately. Compilation or simulated paths do not establish native platform validation.
