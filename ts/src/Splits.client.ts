/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { Coin, StdFee } from "@cosmjs/amino";
import { Cw4Contract, Addr, Config, ExecuteMsg, InstantiateMsg, QueryMsg } from "./Splits.types";
export interface SplitsReadOnlyInterface {
  contractAddress: string;
  config: () => Promise<ConfigResponse>;
  member: ({
    address
  }: {
    address: string;
  }) => Promise<MemberResponse>;
  listMembers: ({
    limit,
    startAfter
  }: {
    limit?: number;
    startAfter?: string;
  }) => Promise<ListMembersResponse>;
}
export class SplitsQueryClient implements SplitsReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.config = this.config.bind(this);
    this.member = this.member.bind(this);
    this.listMembers = this.listMembers.bind(this);
  }

  config = async (): Promise<ConfigResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      config: {}
    });
  };
  member = async ({
    address
  }: {
    address: string;
  }): Promise<MemberResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      member: {
        address
      }
    });
  };
  listMembers = async ({
    limit,
    startAfter
  }: {
    limit?: number;
    startAfter?: string;
  }): Promise<ListMembersResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      list_members: {
        limit,
        start_after: startAfter
      }
    });
  };
}
export interface SplitsInterface extends SplitsReadOnlyInterface {
  contractAddress: string;
  sender: string;
  distribute: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export class SplitsClient extends SplitsQueryClient implements SplitsInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.distribute = this.distribute.bind(this);
  }

  distribute = async (fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      distribute: {}
    }, fee, memo, funds);
  };
}