import { UserConfig, UserConfigFn } from "vite";
import vue from "@vitejs/plugin-vue";
import Components from "unplugin-vue-components/vite";
import { ElementPlusResolver, AntDesignVueResolver } from "unplugin-vue-components/resolvers";
import Gzip from "vite-plugin-compression"
import vueJsx from "@vitejs/plugin-vue-jsx";
import { join } from "path";
const config: UserConfigFn = ({ command }) => {
    const c = <UserConfig>{
        base: "/ka-link",
        server: {
            host: true,
        },
        resolve: {
            alias: {
                src: join(__dirname, "./src"),
                project: join(__dirname, "./"),
            },
        },

        plugins: [
            vue(),
            vueJsx(),
            Gzip({
                threshold: 10,
                deleteOriginFile: true,
                disable: true,
            }),
            Components({
                dts: join(__dirname, "./types/components.d.ts"),
                resolvers: [
                    AntDesignVueResolver({
                        importStyle: false,
                    })
                ],
            }),
        ],

        css: {
            devSourcemap: true,
            preprocessorOptions: {
                scss: {
                    javascriptEnable: true,
                    additionalData: `
                        @use "src/style/global.scss" as *;
                    `,
                },
            },

            postcss: {
                plugins: [
                    // require("postcss-plugin-px2rem")({

                    // }),
                ]
            }
        },
    };

    return c;
};

export default config;
