use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg, SubCommand,
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, Signer};
#[allow(unused_imports)]
use solana_sdk::signer::signers::Signers;
use solana_sdk::transaction::Transaction;
use solana_sdk::system_program;
use borsh::{BorshDeserialize, BorshSerialize,BorshSchema};
use solana_sdk::commitment_config::CommitmentConfig;
use spl_token;
use spl_token_metadata;
use spl_associated_token_account;
#[allow(unused_imports)]
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::borsh::try_from_slice_unchecked;

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
enum StakeInstruction{
    GenerateVault,
    Submit,
    Claim,
    
}

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
struct StakeData{
    timestamp: u64,
    staker: Pubkey,
    active: bool,
}


fn main() {
    let matches = app_from_crate!()
        .subcommand(SubCommand::with_name("generate_vault_address")
            .arg(Arg::with_name("sign")
                .short("s")
                .long("sign")
                .required(true)
                .takes_value(true)
            )
            .arg(Arg::with_name("env")
                .short("e")
                .long("env")
                .required(false)
                .takes_value(true)
            )
            
        )
        
        .subcommand(SubCommand::with_name("submit")
            .arg(Arg::with_name("sign")
                .short("s")
                .long("sign")
                .required(true)
                .takes_value(true)
            )
            .arg(Arg::with_name("env")
                .short("e")
                .long("env")
                .required(false)
                .takes_value(true)
            )
            .arg(Arg::with_name("nft")//the nft user is submitting
                .short("n")
                .long("nft")
                .required(true)
                .takes_value(true)
            )
            .arg(Arg::with_name("reciever")
                .short("r")
                .long("reciever")
                .required(true)
                .takes_value(true)
            )
            .arg(Arg::with_name("nft_recieve")
            
                .long("nft_recieve")
                .required(true)
                .takes_value(true)
            )
        )
        .subcommand(SubCommand::with_name("claim")
            .arg(Arg::with_name("sign")
                .short("s")
                .long("sign")
                .required(true)
                .takes_value(true)
            )
            .arg(Arg::with_name("env")
                .short("e")
                .long("env")
                .required(false)
                .takes_value(true)
            )
            .arg(Arg::with_name("nft")//the nft which user is claiming
                .short("n")
                .long("nft")
                .required(true)
                .takes_value(true)
            )
            .arg(Arg::with_name("nft_one")//the nft which user has submitted
                .long("nft_one")
                .required(true)
                .takes_value(true)
            )
        )
        
        .get_matches();

    let program_id = "9CWuGrchRcg36qLs4Ez6oDpucHpfEKMtSZxxWESzKpjD".parse::<Pubkey>().unwrap();

    
    if let Some(matches) = matches.subcommand_matches("claim") {
        let url = match matches.value_of("env"){
            Some("dev")=>"https://api.devnet.solana.com",
            _=>"https://api.mainnet-beta.solana.com",
        };
        let client = RpcClient::new_with_commitment(url.to_string(),CommitmentConfig::confirmed());
        
        let wallet_path = matches.value_of("sign").unwrap();
        let wallet_keypair = read_keypair_file(wallet_path).expect("Can't open file-wallet");
        let wallet_pubkey = wallet_keypair.pubkey();

        let nft = matches.value_of("nft").unwrap().parse::<Pubkey>().unwrap();
        let nft_one = matches.value_of("nft_one").unwrap().parse::<Pubkey>().unwrap();

        let ( vault, _vault_bump ) = Pubkey::find_program_address(&[&"vault".as_bytes()], &program_id);
        let destanation = spl_associated_token_account::get_associated_token_address(&wallet_pubkey, &nft);
        let source = spl_associated_token_account::get_associated_token_address(&vault, &nft);
        let ( stake_data, _ ) = Pubkey::find_program_address(&[&nft.to_bytes()], &program_id);
        let ( stake_data_one, _ ) = Pubkey::find_program_address(&[&nft_one.to_bytes()], &program_id);


        
        let instarctions = vec![Instruction::new_with_borsh(
            program_id,
            &StakeInstruction::Claim,
            vec![
                AccountMeta::new(wallet_pubkey, true),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(nft, false),
                AccountMeta::new_readonly(nft_one, false),
                AccountMeta::new_readonly(spl_token::id(), false),
                AccountMeta::new_readonly("SysvarRent111111111111111111111111111111111".parse::<Pubkey>().unwrap(), false),
                AccountMeta::new_readonly("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL".parse::<Pubkey>().unwrap(), false),
                AccountMeta::new(stake_data, false),
                AccountMeta::new(stake_data_one, false),
                AccountMeta::new_readonly(vault, false),
                AccountMeta::new(destanation, false),
                AccountMeta::new(source, false),
            ],
        )];
        let mut tx = Transaction::new_with_payer(&instarctions, Some(&wallet_pubkey));
        let recent_blockhash = client.get_latest_blockhash().expect("Can't get blockhash");
        tx.sign(&vec![&wallet_keypair], recent_blockhash);
        let id = client.send_transaction(&tx).expect("Transaction failed.");
        println!("tx id: {:?}", id);
    }

    if let Some(matches) = matches.subcommand_matches("submit") {
        let url = match matches.value_of("env"){
            Some("dev")=>"https://api.devnet.solana.com",
            _=>"https://api.mainnet-beta.solana.com",
        };
        let client = RpcClient::new_with_commitment(url.to_string(),CommitmentConfig::confirmed());
        
        let wallet_path = matches.value_of("sign").unwrap();
        let wallet_keypair = read_keypair_file(wallet_path).expect("Can't open file-wallet");
        let wallet_pubkey = wallet_keypair.pubkey();

        let reciever = matches.value_of("reciever").unwrap().parse::<Pubkey>().unwrap();

        let nft = matches.value_of("nft").unwrap().parse::<Pubkey>().unwrap();
        
        let nft_recieve = matches.value_of("nft_recieve").unwrap().parse::<Pubkey>().unwrap();

        let (metadata,_) =Pubkey::find_program_address(&["metadata".as_bytes(), &spl_token_metadata::ID.to_bytes(), &nft.to_bytes()], &spl_token_metadata::ID);
        let ( vault, _vault_bump ) = Pubkey::find_program_address(&[&"vault".as_bytes()], &program_id);
        let source = spl_associated_token_account::get_associated_token_address(&wallet_pubkey, &nft);
        let destanation = spl_associated_token_account::get_associated_token_address(&vault, &nft);
        let ( stake_data, _ ) = Pubkey::find_program_address(&[&nft.to_bytes()], &program_id);

        let metadata_data = client.get_account_data(&metadata).unwrap();
        let metadata_data_struct: spl_token_metadata::state::Metadata = try_from_slice_unchecked(&metadata_data[..]).unwrap();
        let candy_machine = metadata_data_struct.data.creators.unwrap().first().unwrap().address;

        
        let instarctions = vec![Instruction::new_with_borsh(
            program_id,
            &StakeInstruction::Submit,
            vec![
                AccountMeta::new(wallet_pubkey, true),
                AccountMeta::new(reciever, false),
                AccountMeta::new_readonly(nft, false),
                AccountMeta::new_readonly(nft_recieve, false),
                AccountMeta::new_readonly(metadata, false),
              
                AccountMeta::new_readonly(vault, false),
                AccountMeta::new(source, false),
                AccountMeta::new(destanation, false),

                AccountMeta::new_readonly(spl_token::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly("SysvarRent111111111111111111111111111111111".parse::<Pubkey>().unwrap(), false),
                AccountMeta::new_readonly("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL".parse::<Pubkey>().unwrap(), false),

                AccountMeta::new(stake_data, false),
            ],
        )];
        let mut tx = Transaction::new_with_payer(&instarctions, Some(&wallet_pubkey));
        let recent_blockhash = client.get_latest_blockhash().expect("Can't get blockhash");
        tx.sign(&vec![&wallet_keypair], recent_blockhash);
        let id = client.send_transaction(&tx).expect("Transaction failed.");
        println!("tx id: {:?}", id);
    }

    if let Some(matches) = matches.subcommand_matches("generate_vault_address") {
        let url = match matches.value_of("env"){
            Some("dev")=>"https://api.devnet.solana.com",
            _=>"https://api.mainnet-beta.solana.com",
        };
        let client = RpcClient::new_with_commitment(url.to_string(),CommitmentConfig::confirmed());
        
        let wallet_path = matches.value_of("sign").unwrap();
        let wallet_keypair = read_keypair_file(wallet_path).expect("Can't open file-wallet");
        let wallet_pubkey = wallet_keypair.pubkey();

        let (vault_pda, _) = Pubkey::find_program_address(&["vault".as_bytes()], &program_id);

        let instarctions = vec![Instruction::new_with_borsh(
            program_id,
            &StakeInstruction::GenerateVault,
            vec![
                AccountMeta::new(wallet_pubkey, true),
                AccountMeta::new(system_program::id(), false),
                AccountMeta::new(vault_pda, false),
                AccountMeta::new_readonly("SysvarRent111111111111111111111111111111111".parse::<Pubkey>().unwrap(), false),
            ],
        )];
        let mut tx = Transaction::new_with_payer(&instarctions, Some(&wallet_pubkey));
        let recent_blockhash = client.get_latest_blockhash().expect("Can't get blockhash");
        tx.sign(&vec![&wallet_keypair], recent_blockhash);
        let id = client.send_transaction(&tx).expect("Transaction failed.");
        println!("vault account generated: {:?}", vault_pda);
        println!("tx id: {:?}", id);
    }
   
}
