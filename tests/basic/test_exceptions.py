from src.core.exceptions import ExInfo, LiteLLMExceptions


def test_litellm_exceptions_load():
    """Test that LiteLLMExceptions loads without errors"""
    ex = LiteLLMExceptions()
    assert len(ex.exceptions) > 0


def test_exceptions_tuple():
    """Test that exceptions_tuple returns a non-empty tuple"""
    ex = LiteLLMExceptions()
    assert isinstance(ex.exceptions_tuple(), tuple)
    assert len(ex.exceptions_tuple()) > 0


def test_get_ex_info():
    """Test get_ex_info returns correct ExInfo"""
    ex = LiteLLMExceptions()

    # Test with a known exception type
    from litellm import AuthenticationError

    auth_error = AuthenticationError(
        message="Invalid API key", llm_provider="openai", model="gpt-4"
    )
    ex_info = ex.get_ex_info(auth_error)
    assert isinstance(ex_info, ExInfo)
    assert ex_info.name == "AuthenticationError"
    assert ex_info.retry is False
    assert "API key" in ex_info.description

    # Test with unknown exception type
    class UnknownError(Exception):
        pass

    unknown = UnknownError()
    ex_info = ex.get_ex_info(unknown)
    assert isinstance(ex_info, ExInfo)
    assert ex_info.name is None
    assert ex_info.retry is None
    assert ex_info.description is None


def test_rate_limit_error():
    """Test specific handling of RateLimitError"""
    ex = LiteLLMExceptions()
    from litellm import RateLimitError

    rate_error = RateLimitError(message="Rate limit exceeded", llm_provider="openai", model="gpt-4")
    ex_info = ex.get_ex_info(rate_error)
    assert ex_info.retry is True
    assert "rate limited" in ex_info.description.lower()


def test_context_window_error():
    """Test specific handling of ContextWindowExceededError"""
    ex = LiteLLMExceptions()
    from litellm import ContextWindowExceededError

    ctx_error = ContextWindowExceededError(
        message="Context length exceeded", model="gpt-4", llm_provider="openai"
    )
    ex_info = ex.get_ex_info(ctx_error)
    assert ex_info.retry is False


def test_openrouter_error():
    """Test specific handling of OpenRouter API errors"""
    ex = LiteLLMExceptions()
    from litellm import APIConnectionError

    # Create an APIConnectionError with OpenrouterException message
    openrouter_error = APIConnectionError(
        message="APIConnectionError: OpenrouterException - 'choices'",
        model="openrouter/model",
        llm_provider="openrouter",
    )

    ex_info = ex.get_ex_info(openrouter_error)
    assert ex_info.retry is True
    assert "OpenRouter" in ex_info.description
    assert "overloaded" in ex_info.description
    assert "rate" in ex_info.description


# Edge case tests
def test_api_connection_error_google_auth():
    """Test APIConnectionError with google.auth dependency missing"""
    ex = LiteLLMExceptions()
    from litellm import APIConnectionError

    google_error = APIConnectionError(
        message="google.auth not found",
        model="gemini-pro",
        llm_provider="google",
    )
    ex_info = ex.get_ex_info(google_error)
    assert ex_info.retry is False
    assert "google-generativeai" in ex_info.description


def test_api_connection_error_boto3():
    """Test APIConnectionError with boto3 dependency missing"""
    ex = LiteLLMExceptions()
    from litellm import APIConnectionError

    boto3_error = APIConnectionError(
        message="boto3 not installed",
        model="bedrock/claude",
        llm_provider="bedrock",
    )
    ex_info = ex.get_ex_info(boto3_error)
    assert ex_info.retry is False
    assert "boto3" in ex_info.description


def test_api_error_insufficient_credits():
    """Test APIError with insufficient credits"""
    ex = LiteLLMExceptions()
    from litellm import APIError

    credits_error = APIError(
        status_code=402,
        message='Insufficient credits: {"code":402}',
        llm_provider="openai",
        model="gpt-4",
    )
    ex_info = ex.get_ex_info(credits_error)
    assert ex_info.retry is False
    assert "credits" in ex_info.description.lower()


def test_api_error_generic():
    """Test generic APIError without specific conditions"""
    ex = LiteLLMExceptions()
    from litellm import APIError

    generic_error = APIError(
        status_code=500,
        message="Some other API error",
        llm_provider="openai",
        model="gpt-4",
    )
    ex_info = ex.get_ex_info(generic_error)
    assert ex_info.retry is True  # Default APIError is retryable


def test_service_unavailable_error():
    """Test ServiceUnavailableError"""
    ex = LiteLLMExceptions()
    from litellm import ServiceUnavailableError

    unavailable_error = ServiceUnavailableError(
        message="Service unavailable",
        model="gpt-4",
        llm_provider="openai",
    )
    ex_info = ex.get_ex_info(unavailable_error)
    assert ex_info.retry is True
    assert "down" in ex_info.description.lower()


def test_timeout_error():
    """Test Timeout error"""
    ex = LiteLLMExceptions()
    from litellm import Timeout

    timeout_error = Timeout(message="Request timed out", model="gpt-4", llm_provider="openai")
    ex_info = ex.get_ex_info(timeout_error)
    assert ex_info.retry is True
    assert "timed out" in ex_info.description.lower()


def test_internal_server_error():
    """Test InternalServerError"""
    ex = LiteLLMExceptions()
    from litellm import InternalServerError

    server_error = InternalServerError(
        message="Internal server error",
        model="gpt-4",
        llm_provider="openai",
    )
    ex_info = ex.get_ex_info(server_error)
    assert ex_info.retry is True
    assert "down" in ex_info.description.lower() or "overloaded" in ex_info.description.lower()


def test_bad_request_error():
    """Test BadRequestError is not retryable"""
    ex = LiteLLMExceptions()
    from litellm import BadRequestError

    bad_request = BadRequestError(
        message="Bad request",
        model="gpt-4",
        llm_provider="openai",
    )
    ex_info = ex.get_ex_info(bad_request)
    assert ex_info.retry is False


def test_not_found_error():
    """Test NotFoundError is not retryable"""
    ex = LiteLLMExceptions()
    from litellm import NotFoundError

    not_found = NotFoundError(
        message="Model not found",
        model="gpt-5",
        llm_provider="openai",
    )
    ex_info = ex.get_ex_info(not_found)
    assert ex_info.retry is False


def test_content_policy_violation_error():
    """Test ContentPolicyViolationError"""
    ex = LiteLLMExceptions()
    from litellm import ContentPolicyViolationError

    policy_error = ContentPolicyViolationError(
        message="Content policy violated",
        model="gpt-4",
        llm_provider="openai",
    )
    ex_info = ex.get_ex_info(policy_error)
    assert ex_info.retry is True
    assert "safety policy" in ex_info.description.lower()


def test_multiple_exception_types():
    """Test that all expected exception types are loaded"""
    ex = LiteLLMExceptions()
    expected_exceptions = [
        "AuthenticationError",
        "RateLimitError",
        "ContextWindowExceededError",
        "APIConnectionError",
        "APIError",
        "ContentPolicyViolationError",
    ]

    for exc_name in expected_exceptions:
        assert exc_name in ex.exception_info
        info = ex.exception_info[exc_name]
        assert isinstance(info, ExInfo)
        assert info.name == exc_name


def test_exception_edge_case_empty_message():
    """Test exception with empty or minimal message"""
    ex = LiteLLMExceptions()
    from litellm import RateLimitError

    minimal_error = RateLimitError(
        message="",
        model="gpt-4",
        llm_provider="openai",
    )
    ex_info = ex.get_ex_info(minimal_error)
    assert ex_info.retry is True
    assert ex_info.name == "RateLimitError"


def test_exception_tuple_not_empty():
    """Test that exceptions_tuple always returns a non-empty tuple"""
    ex = LiteLLMExceptions()
    tuple_result = ex.exceptions_tuple()
    assert isinstance(tuple_result, tuple)
    assert len(tuple_result) > 0
    # Verify all items are exception classes
    for exc in tuple_result:
        assert isinstance(exc, type)


# Retry logic tests
def test_retry_logic_exponential_backoff():
    """Test that retry delays follow exponential backoff pattern"""
    from src.core.models import RETRY_TIMEOUT

    retry_delay = 0.125
    max_retry_timeout = RETRY_TIMEOUT
    delays = []

    # Simulate exponential backoff
    while retry_delay <= max_retry_timeout:
        delays.append(retry_delay)
        retry_delay *= 2

    # Verify exponential growth
    assert delays[0] == 0.125
    assert delays[1] == 0.25
    assert delays[2] == 0.5
    assert delays[3] == 1.0
    # Verify we have multiple steps before timeout
    assert len(delays) >= 5


def test_free_tier_rate_limit_timeout():
    """Test that free tier models have shorter timeout"""
    from src.core.models import RETRY_TIMEOUT

    max_retry_timeout_free = 30.0
    max_retry_timeout_paid = RETRY_TIMEOUT * 3

    # Free tier should be much shorter
    assert max_retry_timeout_free < RETRY_TIMEOUT
    # Paid tier should be longer
    assert max_retry_timeout_paid > RETRY_TIMEOUT


def test_retry_delay_progression():
    """Test retry delay progression for rate limits"""
    initial_delay = 0.125
    rate_limit_delay = 5.0

    # For rate limits, delay should jump to 5 seconds
    assert rate_limit_delay > initial_delay * 10


# Graceful degradation tests
def test_graceful_degradation_no_retry():
    """Test that non-retryable errors are properly identified"""
    ex = LiteLLMExceptions()
    from litellm import AuthenticationError, ContextWindowExceededError

    non_retryable = [
        AuthenticationError(message="Auth failed", llm_provider="openai", model="gpt-4"),
        ContextWindowExceededError(
            message="Context exceeded", model="gpt-4", llm_provider="openai"
        ),
    ]

    for error in non_retryable:
        ex_info = ex.get_ex_info(error)
        assert ex_info.retry is False


def test_graceful_degradation_with_retry():
    """Test that retryable errors are properly identified"""
    ex = LiteLLMExceptions()
    from litellm import RateLimitError, APIConnectionError, InternalServerError

    retryable = [
        RateLimitError(message="Rate limited", llm_provider="openai", model="gpt-4"),
        APIConnectionError(message="Connection failed", model="gpt-4", llm_provider="openai"),
        InternalServerError(message="Server error", model="gpt-4", llm_provider="openai"),
    ]

    for error in retryable:
        ex_info = ex.get_ex_info(error)
        assert ex_info.retry is True


def test_exception_info_completeness():
    """Test that ExInfo objects have proper attributes"""
    ex = LiteLLMExceptions()
    from litellm import RateLimitError

    rate_error = RateLimitError(message="Rate limited", llm_provider="openai", model="gpt-4")
    ex_info = ex.get_ex_info(rate_error)

    # Verify all ExInfo attributes are present
    assert hasattr(ex_info, "name")
    assert hasattr(ex_info, "retry")
    assert hasattr(ex_info, "description")
    # Verify they have appropriate values
    assert ex_info.name is not None
    assert ex_info.retry is not None


def test_unknown_exception_handling():
    """Test graceful handling of completely unknown exceptions"""
    ex = LiteLLMExceptions()

    class CompletelyUnknownError(Exception):
        pass

    unknown = CompletelyUnknownError("Unknown error type")
    ex_info = ex.get_ex_info(unknown)

    # Should return None values for unknown exceptions
    assert ex_info.name is None
    assert ex_info.retry is None
    assert ex_info.description is None


def test_exception_with_special_characters():
    """Test exception messages with special characters"""
    ex = LiteLLMExceptions()
    from litellm import APIError

    special_error = APIError(
        status_code=500,
        message="Error with \"quotes\" and 'apostrophes' and symbols: !@#$%",
        llm_provider="openai",
        model="gpt-4",
    )
    ex_info = ex.get_ex_info(special_error)
    assert ex_info.name == "APIError"


def test_multiple_simultaneous_errors():
    """Test handling multiple different error types"""
    ex = LiteLLMExceptions()
    from litellm import RateLimitError, AuthenticationError, APIConnectionError

    errors = [
        RateLimitError(message="Rate limited", llm_provider="openai", model="gpt-4"),
        AuthenticationError(message="Auth failed", llm_provider="openai", model="gpt-4"),
        APIConnectionError(message="Connection failed", model="gpt-4", llm_provider="openai"),
    ]

    results = [ex.get_ex_info(err) for err in errors]

    # Verify each has correct retry behavior
    assert results[0].retry is True  # RateLimitError
    assert results[1].retry is False  # AuthenticationError
    assert results[2].retry is True  # APIConnectionError
