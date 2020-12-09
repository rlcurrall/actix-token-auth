<template>
    <div class="bg-white py-16 px-4 overflow-hidden sm:px-6 lg:px-8 lg:py-24">
        <div class="relative max-w-xl mx-auto">
            <svg
                class="absolute left-full transform translate-x-1/2"
                width="404"
                height="404"
                fill="none"
                viewBox="0 0 404 404"
                aria-hidden="true"
            >
                <defs>
                    <pattern
                        id="85737c0e-0916-41d7-917f-596dc7edfa27"
                        x="0"
                        y="0"
                        width="20"
                        height="20"
                        patternUnits="userSpaceOnUse"
                    >
                        <rect
                            x="0"
                            y="0"
                            width="4"
                            height="4"
                            class="text-gray-200"
                            fill="currentColor"
                        />
                    </pattern>
                </defs>
                <rect
                    width="404"
                    height="404"
                    fill="url(#85737c0e-0916-41d7-917f-596dc7edfa27)"
                />
            </svg>
            <svg
                class="absolute right-full bottom-0 transform -translate-x-1/2"
                width="404"
                height="404"
                fill="none"
                viewBox="0 0 404 404"
                aria-hidden="true"
            >
                <defs>
                    <pattern
                        id="85737c0e-0916-41d7-917f-596dc7edfa27"
                        x="0"
                        y="0"
                        width="20"
                        height="20"
                        patternUnits="userSpaceOnUse"
                    >
                        <rect
                            x="0"
                            y="0"
                            width="4"
                            height="4"
                            class="text-gray-200"
                            fill="currentColor"
                        />
                    </pattern>
                </defs>
                <rect
                    width="404"
                    height="404"
                    fill="url(#85737c0e-0916-41d7-917f-596dc7edfa27)"
                />
            </svg>
            <div class="text-center">
                <h2
                    class="text-3xl font-extrabold tracking-tight text-gray-900 sm:text-4xl"
                >
                    Register Now
                </h2>
                <p class="mt-4 text-lg leading-6 text-gray-500">
                    Nullam risus blandit ac aliquam justo ipsum. Quam mauris
                    volutpat massa dictumst amet. Sapien tortor lacus arcu.
                </p>
            </div>
            <div class="mt-12">
                <form
                    class="grid grid-cols-1 gap-y-6 sm:grid-cols-2 sm:gap-x-8"
                    @submit.prevent="submit"
                >
                    <div class="sm:col-span-2">
                        <label
                            for="full_name"
                            class="block text-sm font-medium text-gray-700"
                        >
                            Full Name
                        </label>
                        <div class="mt-1">
                            <input
                                id="full_name"
                                v-model="full_name"
                                type="text"
                                required
                                aria-required
                                name="full_name"
                                autocomplete="name"
                                class="py-3 px-4 block w-full shadow-sm focus:ring-indigo-500 focus:border-indigo-500 border-gray-300 rounded-md"
                            />
                        </div>
                    </div>
                    <div class="sm:col-span-2">
                        <label
                            for="email"
                            class="block text-sm font-medium text-gray-700"
                        >
                            Email
                        </label>
                        <div class="mt-1">
                            <input
                                id="email"
                                v-model="email"
                                name="email"
                                type="email"
                                required
                                aria-required
                                autocomplete="email"
                                class="py-3 px-4 block w-full shadow-sm focus:ring-indigo-500 focus:border-indigo-500 border-gray-300 rounded-md"
                            />
                        </div>
                    </div>
                    <div class="sm:col-span-2">
                        <label
                            for="password"
                            class="block text-sm font-medium text-gray-700"
                        >
                            Password
                        </label>
                        <div class="mt-1">
                            <input
                                id="password"
                                v-model="password"
                                type="password"
                                name="password"
                                required
                                aria-required
                                autocomplete="password"
                                class="py-3 px-4 block w-full shadow-sm focus:ring-indigo-500 focus:border-indigo-500 border-gray-300 rounded-md"
                            />
                        </div>
                    </div>
                    <div class="sm:col-span-2">
                        <button
                            type="submit"
                            class="w-full inline-flex items-center justify-center px-6 py-3 border border-transparent rounded-md shadow-sm text-base font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                        >
                            Register
                        </button>
                    </div>
                </form>
            </div>
        </div>
    </div>
</template>

<script>
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import http from '../services/http'
export default {
    setup() {
        const router = useRouter()

        const email = ref(null)
        const password = ref(null)
        const full_name = ref(null)

        const submit = async function () {
            try {
                await http.post('register', {
                    email: email.value,
                    password: password.value,
                    full_name: full_name.value,
                })
                router.push({ name: 'login' })
            } catch (e) {
                console.error(e)
            }
        }

        onMounted(async () => {
            try {
                const { status } = await http.get('me')
                if (status === 200) router.push({ name: 'home' })
            } catch (e) {}
        })

        return { email, password, full_name, submit }
    },
}
</script>
