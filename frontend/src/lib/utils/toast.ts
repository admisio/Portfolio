import { toast } from "@zerodevx/svelte-toast";

export const pushErrorText = (text: string) => {
    toast.push(text, {
        theme: {
            '--toastColor': 'mintcream',
            '--toastBackground': '#b91c1c',
            '--toastBarBackground': '#7f1d1d'
        }
    });
};