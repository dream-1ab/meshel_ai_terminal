/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2025-03-22 12:46:29
 * @modify date 2025-03-22 12:46:29
 * @desc [description]
*/


declare module "core/console" {
    export function print(values: any[])
    export function log(values: any[])
    export function warn(values: any[])
    export function info(values: any[])
    export function error(values: any[])
}

declare module "core/threading" {
    export function sleep(millisecond: number)
}