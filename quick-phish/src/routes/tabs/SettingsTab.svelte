<script>
// @ts-nocheck
    import { invoke } from '@tauri-apps/api/core';

    let hideWhitelisted = $state(false);
    let hideBlacklisted = $state(false);

    invoke('get_settings').then((data) => {
        hideWhitelisted = data.whitelist.hide;
        hideBlacklisted = data.blacklist.hide;
        console.log(`Settings: ${JSON.stringify(data)} Hide Whitelisted: ${hideWhitelisted}, Hide Blacklisted: ${hideBlacklisted}`);
    });

</script>

<div class="m-2 h-dvh">
    <h1>Settings</h1>
    <h2>Indicators</h2>
    <h3>Whitelisted</h3>
    <p>Show or hide whitelisted indicators from the Indicator list in the Analysis tab.</p>
    <div class="mr-2 pl-2">
        <div class="flex space-x-2 py-1">
            <input type="radio" value={false} bind:group={hideWhitelisted} id="whitelisted-show" name="whitelisted-sh" class="mt-2 h-4 w-5 rounded-full border-gray-300 text-primary-600 shadow-sm focus:border-primary-300 focus:ring focus:ring-primary-200 focus:ring-opacity-50 focus:ring-offset-0 disabled:cursor-not-allowed disabled:text-gray-400" />
            <label for="whitelisted-show" class="text-sm">
                <div class="font-medium text-gray-700">Show</div>
                <p class="text-gray-500">Show indicators (with slightly different colour) in the indicator list.</p>
            </label>
        </div>
        <div class="flex space-x-2 py-1">
            <input type="radio" value={true} bind:group={hideWhitelisted} id="whitelisted-hide" name="whitelisted-sh" class="mt-2 h-4 w-5 rounded-full border-gray-300 text-primary-600 shadow-sm focus:border-primary-300 focus:ring focus:ring-primary-200 focus:ring-opacity-50 focus:ring-offset-0 disabled:cursor-not-allowed disabled:text-gray-400" />
            <label for="whitelisted-hide" class="text-sm">
                <div class="font-medium text-gray-700">Hide</div>
                <p class="text-gray-500">Hide all whitelisted indicators.</p>
            </label>
        </div>
    </div>

    <h3>Blacklisted</h3>
    <p>Show or hide blacklisted indicators from the Indicator list in the Analysis tab.</p>
    <div class="mr-2 pl-2">
        <div class="flex space-x-2 py-1">
            <input type="radio" value={false} bind:group={hideBlacklisted} id="blacklisted-show" name="blacklisted-sh" class="mt-2 h-4 w-5 rounded-full border-gray-300 text-primary-600 shadow-sm focus:border-primary-300 focus:ring focus:ring-primary-200 focus:ring-opacity-50 focus:ring-offset-0 disabled:cursor-not-allowed disabled:text-gray-400" />
            <label for="blacklisted-show" class="text-sm">
                <div class="font-medium text-gray-700">Show</div>
                <p class="text-gray-500">Show indicators (with slightly different colour) in the indicator list.</p>
            </label>
        </div>
        <div class="flex space-x-2 py-1">
            <input type="radio" value={true} bind:group={hideBlacklisted} id="blacklisted-hide" name="blacklisted-sh" class="mt-2 h-4 w-5 rounded-full border-gray-300 text-primary-600 shadow-sm focus:border-primary-300 focus:ring focus:ring-primary-200 focus:ring-opacity-50 focus:ring-offset-0 disabled:cursor-not-allowed disabled:text-gray-400" />
            <label for="blacklisted-hide" class="text-sm">
                <div class="font-medium text-gray-700">Hide</div>
                <p class="text-gray-500">Hide all blacklisted indicators.</p>
            </label>
        </div>
    </div>

    <h3>Integrations</h3>
    <p>Coming Soon</p>

    <h2>DNS</h2>
    <p>Coming Soon</p>





    
</div>
