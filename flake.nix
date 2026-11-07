{
	description = "Nix development environment for MOSS";
	
	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    	utils.url = "github:numtide/flake-utils";
    	fenix = {
    		url = "github:nix-community/fenix";
    		inputs.nixpkgs.follows = "nixpkgs";
    	};
	};

	outputs = { self, nixpkgs, utils, fenix, ... }: utils.lib.eachDefaultSystem (system: 
		let
			pkgs = nixpkgs.legacyPackages.${system};
			rustToolchain = fenix.packages.${system}.fromToolchainFile {
				file = ./rust-toolchain.toml;
				sha256 = "sha256-+xUr6Al1wZSqX8aEsq7DpuFOK4Ve/i+qTl+K5uWQ9GM=";
			};
		in {
			devShells.default = pkgs.mkShell {
				# Tools
				nativeBuildInputs = with pkgs; [
					# Rust Toolchain
					rustToolchain
					pkg-config
				];

				# Libraries
				buildInputs = with pkgs; [

				];

				# Print banner after entering dev shell
				shellHook = ''
					echo -e "\e[1m\e[38;5;12m• ▌ ▄ ·.       .▄▄ · .▄▄ · \e[0m"
					echo -e "\e[1m\e[38;5;12m·██ ▐███▪▪     ▐█ ▀. ▐█ ▀. \e[0m"
					echo -e "\e[1m\e[38;5;12m▐█ ▌▐▌▐█· ▄█▀▄ ▄▀▀▀█▄▄▀▀▀█▄\e[0m"
					echo -e "\e[1m\e[38;5;12m██ ██▌▐█▌▐█▌.▐▌▐█▄▪▐█▐█▄▪▐█\e[0m"
					echo -e "\e[1m\e[38;5;12m▀▀  █▪▀▀▀ ▀█▄▀▪ ▀▀▀▀  ▀▀▀▀ \e[0m"
					echo -e "\e[1m\e[38;5;1m Rust: \e[0m\e[38;5;8m$(cargo --version)\e[0m"
				'';
			};
		}
	);
}
