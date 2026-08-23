export interface FormatResult {
  output: string;
  logs: string;
}

export interface InitOptions {
  module_or_path: unknown;
}

export interface TexFmtExports {
  main(text: string, config: string): FormatResult;
  version(): string;
}

export function init(options?: InitOptions): Promise<unknown>;
export default init;

export function main(text: string, config: string): FormatResult;
export function format(text: string, config: string): FormatResult;
export function version(): string;
