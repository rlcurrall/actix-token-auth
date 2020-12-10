<template>
    <main class="min-h-screen flex flex-col">
        <nav class="bg-white shadow">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between h-16">
                    <div class="flex">
                        <div class="flex-shrink-0 flex items-center">
                            <img
                                class="block lg:hidden h-8 w-auto"
                                src="https://tailwindui.com/img/logos/workflow-mark-indigo-600.svg"
                                alt="Workflow"
                            />
                            <img
                                class="hidden lg:block h-8 w-auto"
                                src="https://tailwindui.com/img/logos/workflow-logo-indigo-600-mark-gray-800-text.svg"
                                alt="Workflow"
                            />
                        </div>
                        <div class="hidden sm:ml-6 sm:flex sm:space-x-8">
                            <router-link
                                :to="{ name: 'home' }"
                                :class="{
                                    'border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium':
                                        $route.name !== 'home',
                                    'border-indigo-500 text-gray-900 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium':
                                        $route.name === 'home',
                                }"
                            >
                                Home
                            </router-link>
                            <router-link
                                :to="{ name: 'register' }"
                                :class="{
                                    'border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium':
                                        $route.name !== 'register',
                                    'border-indigo-500 text-gray-900 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium':
                                        $route.name === 'register',
                                }"
                            >
                                Register
                            </router-link>
                            <router-link
                                :to="{ name: 'login' }"
                                :class="{
                                    'border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium':
                                        $route.name !== 'login',
                                    'border-indigo-500 text-gray-900 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium':
                                        $route.name === 'login',
                                }"
                            >
                                Login
                            </router-link>
                            <a
                                class="border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium"
                                @click.prevent="logout"
                            >
                                Logout
                            </a>
                        </div>
                    </div>
                    <div class="-mr-2 flex items-center sm:hidden">
                        <!-- Mobile menu button -->
                        <button
                            class="inline-flex items-center justify-center p-2 rounded-md text-gray-400 hover:text-gray-500 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500"
                            aria-expanded="false"
                        >
                            <span class="sr-only">Open main menu</span>
                            <!-- Icon when menu is closed. -->
                            <!--
                                Heroicon name: menu

                                Menu open: "hidden", Menu closed: "block"
                            -->
                            <svg
                                class="block h-6 w-6"
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                                aria-hidden="true"
                                @click="mobileOpen = !mobileOpen"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M4 6h16M4 12h16M4 18h16"
                                />
                            </svg>
                            <!-- Icon when menu is open. -->
                            <!--
                                Heroicon name: x

                                Menu open: "block", Menu closed: "hidden"
                            -->
                            <svg
                                class="hidden h-6 w-6"
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                                aria-hidden="true"
                                @click="mobileOpen = !mobileOpen"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M6 18L18 6M6 6l12 12"
                                />
                            </svg>
                        </button>
                    </div>
                </div>
            </div>

            <!--
                Mobile menu, toggle classes based on menu state.

                Menu open: "block", Menu closed: "hidden"
            -->
            <div
                class="sm:hidden"
                :class="{ block: mobileOpen, hidden: !mobileOpen }"
            >
                <div class="pt-2 pb-3 space-y-1">
                    <router-link
                        :to="{ name: 'home' }"
                        class="bg-indigo-50 border-indigo-500 text-indigo-700 block pl-3 pr-4 py-2 border-l-4 text-base font-medium"
                    >
                        Home
                    </router-link>
                    <router-link
                        :to="{ name: 'register' }"
                        class="border-transparent text-gray-500 hover:bg-gray-50 hover:border-gray-300 hover:text-gray-700 block pl-3 pr-4 py-2 border-l-4 text-base font-medium"
                    >
                        Register
                    </router-link>
                    <router-link
                        :to="{ name: 'login' }"
                        class="border-transparent text-gray-500 hover:bg-gray-50 hover:border-gray-300 hover:text-gray-700 block pl-3 pr-4 py-2 border-l-4 text-base font-medium"
                    >
                        Login
                    </router-link>
                    <a
                        class="border-transparent text-gray-500 hover:bg-gray-50 hover:border-gray-300 hover:text-gray-700 block pl-3 pr-4 py-2 border-l-4 text-base font-medium"
                        @click.prevent="logout"
                    >
                        Logout
                    </a>
                </div>
            </div>
        </nav>
        <!-- <nav
            class="space-x-12 w-full bg-indigo-600 flex items-center justify-center py-4 text-white"
        >
            <router-link
                :to="{ name: 'login' }"
                class="hover:underline focus:underline"
                >Login</router-link
            >
            <router-link
                :to="{ name: 'register' }"
                class="hover:underline focus:underline"
                >Register</router-link
            >
            <router-link
                :to="{ name: 'home' }"
                class="hover:underline focus:underline"
                >Home</router-link
            >
            <a class="hover:underline focus:underline" @click.prevent="logout">
                Logout
            </a>
        </nav> -->

        <router-view />
    </main>
</template>

<script>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import http from './services/http'
export default {
    setup() {
        let router = useRouter()
        let mobileOpen = ref(false)
        let logout = async function () {
            try {
                //// Using Actix Identity works as well
                // let res = await http.get('cookie/logout')
                let res = await http.get('token/logout')
                router.push({ name: 'login' })
            } catch (e) {
                console.error(e)
            }
        }

        return { logout, mobileOpen }
    },
}
</script>
