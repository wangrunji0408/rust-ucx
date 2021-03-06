#@IgnoreInspection BashAddShebang
# This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
# Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


bindingsName='ucx-sys'
rootIncludeFileName='ucx-sys.h'
macosXHomebrewPackageNames='clang-format'
alpineLinuxPackageNames='rsync make gcc linux-headers libunwind-dev linux-grsec-dev'
bindgenAdditionalArguments="--no-prepend-enum-name"
clangAdditionalArguments='-D_GNU_SOURCE'
headersFolderPath='usr/include'
libFolderPath='usr/include'
link='ucm ucs ucp uct'
link_kind='static-nobundle'

final_chance_to_tweak()
{
	sed -i -e 's/#\[derive(Debug, Default, Copy)\]/#[derive(Copy)]/g' "$outputFolderPath"/structs/uct_md_attr.rs


	_fix_bitfield()
	{
		local constant_prefix="$1"
		local bitfield_struct="$2"

		{
			printf '\nimpl %s\n' "$bitfield_struct"
			printf '{\n'

			grep "^pub const $constant_prefix" "$outputFolderPath"/constants/miscellany.rs | \
				sed \
					-e 's/^pub const '"$constant_prefix"'/\tpub const /g' \
					-e 's/: '"$bitfield_struct"'/: Self/g' \

			printf '}\n'
		} >>"$outputFolderPath"/structs/"$bitfield_struct".rs

		grep -v "^pub const $constant_prefix" "$outputFolderPath"/constants/miscellany.rs >"$outputFolderPath"/constants/miscellany.rs.tmp
		rm "$outputFolderPath"/constants/miscellany.rs
		mv "$outputFolderPath"/constants/miscellany.rs.tmp "$outputFolderPath"/constants/miscellany.rs
	}

	_fix_bitfield UCM_EVENT_ ucm_event_type
	_fix_bitfield UCP_EP_PARAM_FIELD_ ucp_ep_params_field
	_fix_bitfield UCP_FEATURE_ ucp_feature
	_fix_bitfield UCP_ATTR_FIELD_ ucp_context_attr_field
	_fix_bitfield UCP_LISTENER_PARAM_FIELD_ ucp_listener_params_field
	_fix_bitfield UCP_MEM_ADVISE_PARAM_FIELD_ ucp_mem_advise_params_field
	_fix_bitfield UCP_MEM_ATTR_FIELD_ ucp_mem_attr_field
	_fix_bitfield UCP_PARAM_FIELD_ ucp_params_field
	_fix_bitfield UCP_WAKEUP_ ucp_wakeup_event_types
	_fix_bitfield UCP_WORKER_ATTR_FIELD_ ucp_worker_attr_field
	_fix_bitfield UCP_WORKER_PARAM_FIELD_ ucp_worker_params_field
	_fix_bitfield UCS_CONFIG_PRINT_ ucs_config_print_flags_t
	_fix_bitfield UCT_CB_FLAG_ uct_cb_flags
	_fix_bitfield UCT_CB_PARAM_FLAG_ uct_cb_param_flags
	_fix_bitfield UCT_EVENT_ uct_iface_event_types
	_fix_bitfield UCT_FLUSH_FLAG_ uct_flush_flags
	_fix_bitfield UCT_IFACE_OPEN_MODE_ uct_iface_open_mode
	_fix_bitfield UCT_PROGRESS_ uct_progress_types
	_fix_bitfield UCT_SEND_FLAG_ uct_msg_flags
	_fix_bitfield UCP_MEM_MAP_PARAM_FIELD_ ucp_mem_map_params_field
	_fix_bitfield UCS_CALLBACKQ_FLAG_ ucs_callbackq_flags
	_fix_bitfield UCT_MD_FLAG_ _bindgen_ty_1
	_fix_bitfield UCP_MEM_MAP_ _bindgen_ty_2
	_fix_bitfield UCT_MD_MEM_ uct_md_mem_flags
	_fix_bitfield UCS_HANDLE_ERROR_ ucs_handle_error_t
	_fix_bitfield UCM_MEM_TYPE_ ucm_mem_type
	_fix_bitfield UCP_EP_PARAMS_FLAGS_ ucp_ep_params_flags_field
	_fix_bitfield UCM_MEM_TYPE_ ucm_mem_type
	_fix_bitfield UCP_STREAM_RECV_FLAG_ ucp_stream_recv_flags_t


	_fix_duplicate_enum_constant()
	{
		local constant_prefix="$1"
		local enum="$2"

		{
			printf '\nimpl %s\n' "$enum"
			printf '{\n'

			grep "^pub const $constant_prefix" "$outputFolderPath"/constants/miscellany.rs | \
				sed \
					-e 's/^pub const '"$constant_prefix"'/\tpub const /g' \
					-e 's/: '"$enum"'/: Self/g' \

			printf '}\n'
		} >>"$outputFolderPath"/enums/"$enum".rs

		grep -v "^pub const $constant_prefix" "$outputFolderPath"/constants/miscellany.rs >"$outputFolderPath"/constants/miscellany.rs.tmp
		rm "$outputFolderPath"/constants/miscellany.rs
		mv "$outputFolderPath"/constants/miscellany.rs.tmp "$outputFolderPath"/constants/miscellany.rs
	}

	_fix_duplicate_enum_constant UCP_DATATYPE_ ucp_dt_type
	_fix_duplicate_enum_constant UCT_ALLOC_METHOD_ uct_alloc_method_t


	_fix_last_in_enum()
	{
		local enum
		for enum in "$@"
		do
			grep -v "^\t.*_LAST" "$outputFolderPath"/enums/"$enum".rs >"$outputFolderPath"/enums/"$enum".rs.tmp
			rm "$outputFolderPath"/enums/"$enum".rs
			mv "$outputFolderPath"/enums/"$enum".rs.tmp "$outputFolderPath"/enums/"$enum".rs
		done
	}

	_fix_last_in_enum \
		ucp_atomic_fetch_op_t \
		ucs_async_mode_t \
		ucs_stats_formats_t \
		ucs_ternary_value \
		uct_am_trace_type \
		uct_device_type_t \
		ucs_log_level_t \
		ucs_thread_mode_t \
		ucp_atomic_post_op_t \
		uct_memory_type_t


	_fix_non_option_callback()
	{
		local callback
		for callback in "$@"
		do
			sed -i -e "s/Option<unsafe/unsafe/g" -e 's/>;$/;/g' "$outputFolderPath"/types/"$callback".rs
		done
	}

	# These callback types can be null:-
	# ucp_err_handler_cb_t
	# ucp_request_init_callback_t
	# ucp_request_cleanup_callback_t

	_fix_non_option_callback \
		ucm_event_callback_t \
		ucp_listener_accept_callback_t \
		ucp_listener_accept_handler_t \
		ucp_send_callback_t \
		ucp_stream_recv_callback_t \
		ucp_tag_recv_callback_t \
		ucs_async_event_cb_t \
		ucs_callback_t \
		uct_am_callback_t \
		uct_completion_callback_t \
		uct_desc_release_callback_t \
		uct_error_handler_t \
		uct_pack_callback_t \
		uct_pending_callback_t \
		uct_pending_purge_callback_t \
		uct_sockaddr_conn_request_callback_t \
		uct_tag_unexp_eager_cb_t \
		uct_tag_unexp_rndv_cb_t \
		uct_unpack_callback_t

	# The above changes to `ucs_callback_t` affect the sentinel in `ucs_callbackq_elem`.
	sed -i -e "s/pub cb: ucs_callback_t/pub cb: Option<ucs_callback_t>/g" "$outputFolderPath"/structs/ucs_callbackq_elem.rs

	_fix_non_option_callback_on_struct()
	{
		local struct
		for struct in "$@"
		do
			sed -i -e "s/Option<unsafe/unsafe/g" -e 's/>,$/,/g' "$outputFolderPath"/structs/"$struct".rs
		done
	}

	_fix_non_option_callback_on_struct \
		uct_tag_context \
		uct_iface_ops
}
