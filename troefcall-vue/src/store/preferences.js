import { defineStore } from "pinia";

const LIGHTMODE = 'lightmode';

export const usePreferencesStore = defineStore({
    id: 'pref',
    // state: () => ({
    // }),
    getters: {
        getLightmodePreference: () => {
            return localStorage.getItem(LIGHTMODE) === 'true';
        }
    },
    actions: {
        setLightmodePreference(lightMode) {
            localStorage.setItem(LIGHTMODE, lightMode);
        }
    },
});
